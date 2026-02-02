//! A2A (Agent-to-Agent) Client
//!
//! Implements the A2A JSON-RPC protocol for communicating with A2UI agents.
//! Uses SSE streaming for receiving progressive UI updates.

use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::message::A2uiMessage;
use super::sse::{SseClient, SseEvent};

/// A2A extension URI for A2UI protocol
pub const A2UI_EXTENSION_URI: &str = "https://a2ui.org/a2a-extension/a2ui/v0.8";

/// A2A client for communicating with agents
pub struct A2aClient {
    url: String,
    auth_token: Option<String>,
    request_id: u64,
    task_id: Option<String>,
    context_id: Option<String>,
}

impl A2aClient {
    /// Create a new A2A client
    pub fn new(url: impl Into<String>) -> Self {
        A2aClient {
            url: url.into(),
            auth_token: None,
            request_id: 1,
            task_id: None,
            context_id: None,
        }
    }

    /// Set authentication token
    pub fn with_auth(mut self, token: impl Into<String>) -> Self {
        self.auth_token = Some(token.into());
        self
    }

    /// Get current task ID
    pub fn task_id(&self) -> Option<&str> {
        self.task_id.as_deref()
    }

    /// Get current context ID
    pub fn context_id(&self) -> Option<&str> {
        self.context_id.as_deref()
    }

    /// Send a message and receive streaming A2UI updates
    pub fn message_stream(&mut self, content: &str) -> Result<A2aEventStream, String> {
        let message_id = Uuid::new_v4().to_string();
        let context_id = self
            .context_id
            .clone()
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        // Build JSON-RPC request
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "message/stream".to_string(),
            params: MessageParams {
                configuration: None,
                metadata: None,
                message: Message {
                    message_id,
                    role: "user".to_string(),
                    parts: vec![Part::Text {
                        text: content.to_string(),
                    }],
                    context_id: context_id.clone(),
                    extensions: vec![A2UI_EXTENSION_URI.to_string()],
                },
            },
            id: self.request_id,
        };

        self.request_id += 1;
        self.context_id = Some(context_id);

        let body = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        // Build SSE client
        let mut client = SseClient::new(&self.url)
            .header("X-A2A-Extensions", A2UI_EXTENSION_URI);

        if let Some(token) = &self.auth_token {
            client = client.auth(token);
        }

        let rx = client.post(&body)?;

        Ok(A2aEventStream {
            receiver: rx,
            client_task_id: self.task_id.clone(),
            client_context_id: self.context_id.clone(),
        })
    }

    /// Send a user action back to the agent
    pub fn send_action(
        &mut self,
        action_name: &str,
        source_component_id: &str,
        context: HashMap<String, Value>,
    ) -> Result<(), String> {
        let Some(task_id) = &self.task_id else {
            return Err("No active task to send action to".to_string());
        };

        let Some(context_id) = &self.context_id else {
            return Err("No active context".to_string());
        };

        let message_id = Uuid::new_v4().to_string();

        // Build A2UI event
        let a2ui_event = A2uiEvent {
            action_name: action_name.to_string(),
            source_component_id: source_component_id.to_string(),
            timestamp: chrono_now(),
            resolved_context: context,
        };

        // Wrap in A2A message
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "message/send".to_string(),
            params: MessageParams {
                configuration: None,
                metadata: None,
                message: Message {
                    message_id,
                    role: "user".to_string(),
                    parts: vec![Part::Data {
                        data: serde_json::json!({ "a2uiEvent": a2ui_event }),
                    }],
                    context_id: context_id.clone(),
                    extensions: vec![A2UI_EXTENSION_URI.to_string()],
                },
            },
            id: self.request_id,
        };

        self.request_id += 1;

        let body = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        // Send non-streaming request
        let mut req = ureq::post(&self.url)
            .set("Content-Type", "application/json")
            .set("X-A2A-Extensions", A2UI_EXTENSION_URI);

        if let Some(token) = &self.auth_token {
            req = req.set("Authorization", &format!("Bearer {}", token));
        }

        req.send_string(&body)
            .map_err(|e| format!("Failed to send action: {}", e))?;

        Ok(())
    }

    /// Update task ID from received event
    pub fn set_task_id(&mut self, task_id: impl Into<String>) {
        self.task_id = Some(task_id.into());
    }
}

/// Stream of A2A events
pub struct A2aEventStream {
    receiver: Receiver<SseEvent>,
    client_task_id: Option<String>,
    client_context_id: Option<String>,
}

impl A2aEventStream {
    /// Receive next A2UI message from stream
    /// Returns None when stream ends
    pub fn next(&mut self) -> Option<A2aStreamEvent> {
        loop {
            match self.receiver.recv() {
                Ok(SseEvent::Data(data)) => {
                    // Parse JSON-RPC response
                    match serde_json::from_str::<JsonRpcResponse>(&data) {
                        Ok(response) => {
                            if let Some(error) = response.error {
                                return Some(A2aStreamEvent::Error(format!(
                                    "JSON-RPC error: {} - {}",
                                    error.code, error.message
                                )));
                            }

                            if let Some(result) = response.result {
                                return self.process_result(result);
                            }
                        }
                        Err(e) => {
                            // Try parsing as direct A2UI message
                            match serde_json::from_str::<A2uiMessage>(&data) {
                                Ok(msg) => return Some(A2aStreamEvent::A2uiMessage(msg)),
                                Err(_) => {
                                    // Log parse error but continue
                                    continue;
                                }
                            }
                        }
                    }
                }
                Ok(SseEvent::Comment(_)) => {
                    // Keep-alive, continue
                    continue;
                }
                Ok(SseEvent::Error(e)) => {
                    return Some(A2aStreamEvent::Error(e));
                }
                Ok(SseEvent::Done) => {
                    return None;
                }
                Err(_) => {
                    // Channel closed
                    return None;
                }
            }
        }
    }

    fn process_result(&mut self, result: ResultValue) -> Option<A2aStreamEvent> {
        match result {
            ResultValue::Task(task) => {
                // Update task ID
                self.client_task_id = Some(task.id.clone());
                Some(A2aStreamEvent::TaskStatus {
                    task_id: task.id,
                    state: task.status.state,
                })
            }
            ResultValue::Event(event) => {
                // Check for A2UI messages in data
                if let Some(data) = event.data {
                    eprintln!("[A2A] Event data: {}", serde_json::to_string_pretty(&data).unwrap_or_default());

                    // Try to parse as A2UI message
                    match serde_json::from_value::<A2uiMessage>(data.clone()) {
                        Ok(msg) => {
                            eprintln!("[A2A] Parsed A2uiMessage directly: {:?}", msg);
                            return Some(A2aStreamEvent::A2uiMessage(msg));
                        }
                        Err(e) => {
                            eprintln!("[A2A] Direct A2uiMessage parse failed: {}", e);
                        }
                    }

                    // Check for nested A2UI message keys
                    if let Some(obj) = data.as_object() {
                        for key in [
                            "beginRendering",
                            "surfaceUpdate",
                            "dataModelUpdate",
                            "deleteSurface",
                        ] {
                            if obj.contains_key(key) {
                                if let Ok(msg) = serde_json::from_value::<A2uiMessage>(data.clone())
                                {
                                    return Some(A2aStreamEvent::A2uiMessage(msg));
                                }
                            }
                        }
                    }
                }
                None
            }
            ResultValue::Other(_) => None,
        }
    }

    /// Get current task ID
    pub fn task_id(&self) -> Option<&str> {
        self.client_task_id.as_deref()
    }
}

/// Event from A2A stream
#[derive(Debug, Clone)]
pub enum A2aStreamEvent {
    /// A2UI protocol message
    A2uiMessage(A2uiMessage),
    /// Task status update
    TaskStatus { task_id: String, state: String },
    /// Error
    Error(String),
}

// ============================================================================
// JSON-RPC types
// ============================================================================

#[derive(Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    params: MessageParams,
    id: u64,
}

#[derive(Serialize)]
struct MessageParams {
    configuration: Option<Value>,
    metadata: Option<Value>,
    message: Message,
}

#[derive(Serialize)]
struct Message {
    #[serde(rename = "messageId")]
    message_id: String,
    role: String,
    parts: Vec<Part>,
    #[serde(rename = "contextId")]
    context_id: String,
    extensions: Vec<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
enum Part {
    Text { text: String },
    Data { data: Value },
}

#[derive(Serialize)]
struct A2uiEvent {
    #[serde(rename = "actionName")]
    action_name: String,
    #[serde(rename = "sourceComponentId")]
    source_component_id: String,
    timestamp: String,
    #[serde(rename = "resolvedContext")]
    resolved_context: HashMap<String, Value>,
}

#[derive(Deserialize)]
struct JsonRpcResponse {
    result: Option<ResultValue>,
    error: Option<JsonRpcError>,
}

#[derive(Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ResultValue {
    Task(TaskResult),
    Event(EventResult),
    Other(Value),
}

#[derive(Deserialize)]
struct TaskResult {
    kind: String,
    id: String,
    #[serde(rename = "contextId")]
    context_id: Option<String>,
    status: TaskStatus,
}

#[derive(Deserialize)]
struct TaskStatus {
    state: String,
}

#[derive(Deserialize)]
struct EventResult {
    kind: String,
    #[serde(rename = "taskId")]
    task_id: Option<String>,
    data: Option<Value>,
}

/// Get current timestamp in ISO format
fn chrono_now() -> String {
    // Simple timestamp without chrono dependency
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}", duration.as_secs())
}
