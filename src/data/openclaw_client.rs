//! OpenClaw Gateway client implementation.
//!
//! This module provides a client for interacting with the OpenClaw Gateway,
//! a local AI assistant platform that supports multiple messaging channels.

use async_stream::stream;
use futures::{FutureExt, SinkExt, StreamExt};
use moly_kit::aitk::utils::asynchronous::sleep;
use moly_kit::prelude::*;
use serde::Serialize;
use serde_json::Value;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};

/// OpenClaw protocol request wrapper.
#[derive(Debug, Clone, Serialize)]
struct Request<T> {
    r#type: &'static str,
    id: String,
    method: String,
    params: T,
}

impl<T> Request<T> {
    fn new(method: &str, params: T) -> Self {
        Self {
            r#type: "req",
            id: Uuid::new_v4().to_string(),
            method: method.to_string(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConnectParams {
    min_protocol: u32,
    max_protocol: u32,
    role: &'static str,
    scopes: Vec<&'static str>,
    client: ClientInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<AuthInfo>,
}

#[derive(Debug, Clone, Serialize)]
struct ClientInfo {
    id: &'static str,
    version: &'static str,
    platform: &'static str,
    mode: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct AuthInfo {
    token: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AgentParams {
    message: String,
    idempotency_key: String,
    agent_id: &'static str,
}

#[derive(Debug, Clone)]
struct OpenClawClientInner {
    url: String,
    token: Option<String>,
}

/// A client for interacting with the OpenClaw Gateway.
#[derive(Debug)]
pub struct OpenClawClient(Arc<RwLock<OpenClawClientInner>>);

impl Clone for OpenClawClient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl From<OpenClawClientInner> for OpenClawClient {
    fn from(inner: OpenClawClientInner) -> Self {
        Self(Arc::new(RwLock::new(inner)))
    }
}

impl OpenClawClient {
    /// Creates a new OpenClaw client with the given Gateway URL.
    pub fn new(url: String) -> Self {
        OpenClawClientInner { url, token: None }.into()
    }

    /// Sets the authentication token for the client.
    pub fn set_key(&mut self, token: &str) -> Result<(), &'static str> {
        self.0
            .write()
            .map_err(|_| "OpenClaw client lock poisoned")?
            .token = Some(token.to_string());
        Ok(())
    }

    fn build_connect_request(token: Option<&str>) -> Request<ConnectParams> {
        Request::new(
            "connect",
            ConnectParams {
                min_protocol: 3,
                max_protocol: 3,
                role: "operator",
                scopes: vec!["operator.read", "operator.write"],
                client: ClientInfo {
                    id: "gateway-client",
                    version: "1.0.0",
                    platform: "desktop",
                    mode: "cli",
                },
                auth: token.map(|t| AuthInfo { token: t.to_string() }),
            },
        )
    }

    fn build_agent_request(message: String) -> Request<AgentParams> {
        Request::new(
            "agent",
            AgentParams {
                message,
                idempotency_key: Uuid::new_v4().to_string(),
                agent_id: "main",
            },
        )
    }

    fn build_history_message(messages: &[Message]) -> String {
        let mut combined = String::new();
        let start = messages.len().saturating_sub(MAX_HISTORY_MESSAGES);
        for message in &messages[start..] {
            let role = match message.from {
                EntityId::User => "user",
                EntityId::System => "system",
                EntityId::Bot(_) => "assistant",
                EntityId::Tool | EntityId::App => continue,
            };

            let text = message.content.text.trim();
            if text.is_empty() {
                continue;
            }

            if !combined.is_empty() {
                combined.push_str("\n\n");
            }
            combined.push_str(role);
            combined.push_str(": ");
            combined.push_str(text);
        }

        if combined.is_empty() {
            let fallback = messages
                .iter()
                .rev()
                .find(|m| matches!(m.from, EntityId::User))
                .map(|m| m.content.text.trim())
                .filter(|text| !text.is_empty());

            if let Some(text) = fallback {
                combined.push_str(text);
            }
        }

        combined
    }
}

#[cfg(not(target_arch = "wasm32"))]
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_HISTORY_MESSAGES: usize = 32;

/// Result of processing a WebSocket message.
enum ProcessResult {
    Continue,
    Yield(MessageContent),
    Error(ClientError),
    SendConnect,
    SendAgent,
    Done,
}

/// Processes an event message from OpenClaw.
fn process_event(
    event: &str,
    payload: Option<&Value>,
    content: &mut MessageContent,
) -> ProcessResult {
    match event {
        "connect.challenge" => {
            log::debug!("OpenClaw: received connect.challenge");
            ProcessResult::SendConnect
        }
        "agent.text" | "agent.content" => {
            if let Some(text) = payload.and_then(|p| p["text"].as_str()) {
                content.text = text.to_string();
                ProcessResult::Yield(content.clone())
            } else {
                ProcessResult::Continue
            }
        }
        "agent.text.delta" | "agent.content.delta" => {
            let delta = payload
                .and_then(|p| p["delta"].as_str().or(p["text"].as_str()));
            if let Some(d) = delta {
                content.text.push_str(d);
                ProcessResult::Yield(content.clone())
            } else {
                ProcessResult::Continue
            }
        }
        "agent" => {
            if let Some(delta) = payload.and_then(|p| p["data"]["delta"].as_str()) {
                content.text.push_str(delta);
                ProcessResult::Yield(content.clone())
            } else {
                ProcessResult::Continue
            }
        }
        "agent.done" | "agent.complete" | "agent.end" | "chat" => {
            if event == "chat" {
                let state = payload.and_then(|p| p["state"].as_str());
                if state != Some("done") && state != Some("complete") {
                    return ProcessResult::Continue;
                }
            }
            ProcessResult::Done
        }
        "agent.error" => {
            let msg = payload
                .and_then(|p| p["message"].as_str().or(p["error"].as_str()))
                .unwrap_or("Unknown error");
            ProcessResult::Error(ClientError::new(
                ClientErrorKind::Response,
                format!("OpenClaw error: {}", msg),
            ))
        }
        _ => {
            log::trace!("OpenClaw: ignoring event '{}'", event);
            ProcessResult::Continue
        }
    }
}

/// Processes a response message from OpenClaw.
fn process_response(
    json: &Value,
    content: &mut MessageContent,
    connected: &mut bool,
) -> ProcessResult {
    let ok = json["ok"].as_bool().unwrap_or(false);
    if !ok {
        let msg = json["error"].as_str().unwrap_or("Unknown error");
        return ProcessResult::Error(ClientError::new(
            ClientErrorKind::Response,
            format!("OpenClaw error: {}", msg),
        ));
    }

    let payload = match json.get("payload") {
        Some(p) => p,
        None => return ProcessResult::Continue,
    };

    // Handle hello-ok (connection successful)
    if payload["type"].as_str() == Some("hello-ok") && !*connected {
        *connected = true;
        log::debug!("OpenClaw: connected, sending agent request");
        return ProcessResult::SendAgent;
    }

    // Handle agent response completion
    let status = payload["status"].as_str();
    if status == Some("ok") || status == Some("completed") {
        // Prefer streaming content if available
        if !content.text.is_empty() {
            return ProcessResult::Done;
        }
        // Extract from result.payloads[].text
        if let Some(payloads) = payload["result"]["payloads"].as_array() {
            for p in payloads {
                if let Some(text) = p["text"].as_str() {
                    content.text.push_str(text);
                }
            }
            if !content.text.is_empty() {
                return ProcessResult::Done;
            }
        }
    }

    ProcessResult::Continue
}

impl BotClient for OpenClawClient {
    fn bots(&mut self) -> BoxPlatformSendFuture<'static, ClientResult<Vec<Bot>>> {
        let bot = Bot {
            id: BotId::new("openclaw/assistant"),
            name: "OpenClaw Assistant".to_string(),
            avatar: EntityAvatar::Text("🦞".into()),
            capabilities: BotCapabilities::new().with_capabilities([BotCapability::TextInput]),
        };
        Box::pin(async move { ClientResult::new_ok(vec![bot]) })
    }

    fn clone_box(&self) -> Box<dyn BotClient> {
        Box::new(self.clone())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn send(
        &mut self,
        _bot_id: &BotId,
        messages: &[Message],
        _tools: &[Tool],
    ) -> BoxPlatformSendStream<'static, ClientResult<MessageContent>> {
        let inner = match self.0.read() {
            Ok(inner) => inner.clone(),
            Err(_) => {
                let stream = stream! {
                    yield ClientError::new(
                        ClientErrorKind::Unknown,
                        "OpenClaw client lock poisoned".to_string(),
                    ).into();
                };
                return Box::pin(stream);
            }
        };
        let history_message = Self::build_history_message(messages);

        let stream = stream! {
            log::debug!("OpenClaw: connecting to {}", inner.url);

            let (ws_stream, _) = match connect_async(&inner.url).await {
                Ok(r) => r,
                Err(e) => {
                    yield ClientError::new(
                        ClientErrorKind::Network,
                        format!("Failed to connect to OpenClaw Gateway: {}", e),
                    ).into();
                    return;
                }
            };

            let (mut write, read) = ws_stream.split();
            let mut read = read.fuse();
            let mut content = MessageContent::default();
            let mut connected = false;
            let mut history_message = history_message;
            let handshake_deadline = std::pin::pin!(sleep(HANDSHAKE_TIMEOUT));
            let mut handshake_deadline = handshake_deadline.fuse();

            let connect_req = Self::build_connect_request(inner.token.as_deref());
            let connect_json = match serde_json::to_string(&connect_req) {
                Ok(j) => j,
                Err(e) => {
                    yield ClientError::new(
                        ClientErrorKind::Format,
                        format!("Failed to serialize request: {}", e),
                    ).into();
                    return;
                }
            };
            if let Err(e) = write.send(WsMessage::Text(connect_json.into())).await {
                yield ClientError::new(
                    ClientErrorKind::Network,
                    format!("Failed to send request: {}", e),
                ).into();
                return;
            }

            loop {
                let msg_result = futures::select! {
                    _ = handshake_deadline => {
                        if !connected {
                            yield ClientError::new(
                                ClientErrorKind::Network,
                                "Timed out waiting for OpenClaw handshake".to_string(),
                            ).into();
                            return;
                        }
                        None
                    }
                    msg = read.next() => msg,
                };

                let msg_result = match msg_result {
                    Some(result) => result,
                    None => {
                        log::debug!("OpenClaw: connection closed");
                        break;
                    }
                };

                let result = match msg_result {
                    Ok(WsMessage::Text(text)) => {
                        let json: Value = match serde_json::from_str(&text) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                        match json["type"].as_str().unwrap_or("") {
                            "event" => {
                                let event = json["event"].as_str().unwrap_or("");
                                process_event(event, json.get("payload"), &mut content)
                            }
                            "res" => process_response(&json, &mut content, &mut connected),
                            t => {
                                log::trace!("OpenClaw: ignoring message type '{}'", t);
                                ProcessResult::Continue
                            }
                        }
                    }
                    Ok(WsMessage::Close(_)) => {
                        log::debug!("OpenClaw: connection closed");
                        break;
                    }
                    Ok(_) => ProcessResult::Continue,
                    Err(e) => ProcessResult::Error(ClientError::new(
                        ClientErrorKind::Network,
                        format!("WebSocket error: {}", e),
                    )),
                };

                match result {
                    ProcessResult::Continue => {}
                    ProcessResult::Yield(c) => yield ClientResult::new_ok(c),
                    ProcessResult::Error(e) => {
                        yield e.into();
                        break;
                    }
                    ProcessResult::SendConnect => {
                        if connected {
                            continue;
                        }
                        let req = Self::build_connect_request(inner.token.as_deref());
                        let json = match serde_json::to_string(&req) {
                            Ok(j) => j,
                            Err(e) => {
                                yield ClientError::new(
                                    ClientErrorKind::Format,
                                    format!("Failed to serialize request: {}", e),
                                ).into();
                                return;
                            }
                        };
                        if let Err(e) = write.send(WsMessage::Text(json.into())).await {
                            yield ClientError::new(
                                ClientErrorKind::Network,
                                format!("Failed to send request: {}", e),
                            ).into();
                            return;
                        }
                    }
                    ProcessResult::SendAgent => {
                        let req =
                            Self::build_agent_request(std::mem::take(&mut history_message));
                        let json = match serde_json::to_string(&req) {
                            Ok(j) => j,
                            Err(e) => {
                                yield ClientError::new(
                                    ClientErrorKind::Format,
                                    format!("Failed to serialize request: {}", e),
                                ).into();
                                return;
                            }
                        };
                        if let Err(e) = write.send(WsMessage::Text(json.into())).await {
                            yield ClientError::new(
                                ClientErrorKind::Network,
                                format!("Failed to send request: {}", e),
                            ).into();
                            return;
                        }
                    }
                    ProcessResult::Done => {
                        yield ClientResult::new_ok(content.clone());
                        break;
                    }
                }
            }
        };

        Box::pin(stream)
    }

    #[cfg(target_arch = "wasm32")]
    fn send(
        &mut self,
        _bot_id: &BotId,
        _messages: &[Message],
        _tools: &[Tool],
    ) -> BoxPlatformSendStream<'static, ClientResult<MessageContent>> {
        let inner = match self.0.read() {
            Ok(inner) => inner.clone(),
            Err(_) => {
                let stream = stream! {
                    yield ClientError::new(
                        ClientErrorKind::Unknown,
                        "OpenClaw client lock poisoned".to_string(),
                    ).into();
                };
                return Box::pin(stream);
            }
        };
        let stream = stream! {
            let url = inner.url.replace("ws://", "http://").replace("wss://", "https://");
            let content = MessageContent {
                text: format!(
                    "OpenClaw Gateway is not supported on web platform.\n\n\
                    Please use the desktop version of Moly or access OpenClaw Web UI directly: {}",
                    url
                ),
                ..Default::default()
            };
            yield ClientResult::new_ok(content);
        };
        Box::pin(stream)
    }
}
