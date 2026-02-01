//! OpenClaw Gateway client implementation.
//!
//! This module provides a client for interacting with the OpenClaw Gateway,
//! a local AI assistant platform that supports multiple messaging channels.
//!
//! OpenClaw Gateway uses WebSocket with a custom protocol for communication.

use async_stream::stream;
use futures::{SinkExt, StreamExt};
use moly_kit::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};

/// OpenClaw request structure
/// Format: {type:"req", id, method, params}
#[derive(Debug, Clone, Serialize)]
struct OpenClawRequest<T> {
    r#type: &'static str,
    id: String,
    method: String,
    params: T,
}

impl<T> OpenClawRequest<T> {
    fn new(method: &str, params: T) -> Self {
        Self {
            r#type: "req",
            id: Uuid::new_v4().to_string(),
            method: method.to_string(),
            params,
        }
    }
}

/// OpenClaw connect parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConnectParams {
    min_protocol: u32,
    max_protocol: u32,
    role: String,
    scopes: Vec<String>,
    client: ClientInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    auth: Option<AuthInfo>,
}

#[derive(Debug, Clone, Serialize)]
struct ClientInfo {
    id: String,
    version: String,
    platform: String,
    mode: String,
}

#[derive(Debug, Clone, Serialize)]
struct AuthInfo {
    token: String,
}

/// OpenClaw agent parameters
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct AgentParams {
    message: String,
    idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<String>,
}

/// Inner state of the OpenClaw client
#[derive(Debug, Clone)]
struct OpenClawClientInner {
    url: String,
    token: Option<String>,
}

/// A client for interacting with the OpenClaw Gateway.
///
/// OpenClaw is a local AI assistant platform that provides:
/// - Multi-channel messaging (WhatsApp, Telegram, Slack, etc.)
/// - Browser control and automation
/// - Canvas visualization workspace
/// - Skill system for extensibility
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
    ///
    /// # Arguments
    /// * `url` - The WebSocket URL of the OpenClaw Gateway (e.g., "ws://127.0.0.1:18789")
    pub fn new(url: String) -> Self {
        OpenClawClientInner { url, token: None }.into()
    }

    /// Sets the authentication token for the client.
    pub fn set_key(&mut self, token: &str) -> Result<(), &'static str> {
        self.0.write().unwrap().token = Some(token.to_string());
        Ok(())
    }
}

impl BotClient for OpenClawClient {
    fn bots(&mut self) -> BoxPlatformSendFuture<'static, ClientResult<Vec<Bot>>> {
        let bot = Bot {
            id: BotId::new("openclaw/assistant"),
            name: "OpenClaw Assistant".to_string(),
            avatar: EntityAvatar::Text("🦞".into()),
            capabilities: BotCapabilities::new().with_capabilities([
                BotCapability::TextInput,
                BotCapability::AttachmentInput,
            ]),
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
        let inner = self.0.read().unwrap().clone();

        // Get the last user message
        let last_message = messages
            .iter()
            .rev()
            .find(|m| matches!(m.from, EntityId::User))
            .map(|m| m.content.text.clone())
            .unwrap_or_default();

        let stream = stream! {
            let ws_url = inner.url.clone();

            log::debug!("OpenClaw connecting to: {}", ws_url);

            // Connect to WebSocket
            let ws_result = connect_async(&ws_url).await;
            let (ws_stream, _) = match ws_result {
                Ok(r) => r,
                Err(e) => {
                    yield ClientError::new(
                        ClientErrorKind::Network,
                        format!("Failed to connect to OpenClaw Gateway: {}", e),
                    ).into();
                    return;
                }
            };

            let (mut write, mut read) = ws_stream.split();
            let mut content = MessageContent::default();
            let mut connected = false;

            // Read messages and handle protocol
            while let Some(msg_result) = read.next().await {
                match msg_result {
                    Ok(WsMessage::Text(text)) => {
                        let text_str = text.to_string();
                        log::debug!("OpenClaw received: {}", text_str);

                        // Parse as generic JSON
                        let json: serde_json::Value = match serde_json::from_str(&text_str) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                        let msg_type = json.get("type").and_then(|t| t.as_str()).unwrap_or("");

                        match msg_type {
                            // Handle events
                            "event" => {
                                let event_name = json.get("event").and_then(|e| e.as_str()).unwrap_or("");
                                let payload = json.get("payload");

                                match event_name {
                                    // Connection challenge - send connect request
                                    "connect.challenge" => {
                                        log::debug!("OpenClaw received connect.challenge, sending connect request");
                                        log::debug!("OpenClaw token present: {}", inner.token.is_some());

                                        let connect_request = OpenClawRequest::new(
                                            "connect",
                                            ConnectParams {
                                                min_protocol: 3,
                                                max_protocol: 3,
                                                role: "operator".to_string(),
                                                scopes: vec![
                                                    "operator.read".to_string(),
                                                    "operator.write".to_string(),
                                                ],
                                                client: ClientInfo {
                                                    id: "gateway-client".to_string(),
                                                    version: "1.0.0".to_string(),
                                                    platform: "desktop".to_string(),
                                                    mode: "cli".to_string(),
                                                },
                                                auth: inner.token.as_ref().map(|t| AuthInfo { token: t.clone() }),
                                            },
                                        );

                                        let connect_json = match serde_json::to_string(&connect_request) {
                                            Ok(json) => json,
                                            Err(e) => {
                                                yield ClientError::new(
                                                    ClientErrorKind::Format,
                                                    format!("Failed to serialize connect request: {}", e),
                                                ).into();
                                                return;
                                            }
                                        };

                                        log::debug!("OpenClaw sending connect: {}", connect_json);

                                        if let Err(e) = write.send(WsMessage::Text(connect_json.into())).await {
                                            yield ClientError::new(
                                                ClientErrorKind::Network,
                                                format!("Failed to send connect request: {}", e),
                                            ).into();
                                            return;
                                        }
                                    }
                                    // Agent text streaming events
                                    "agent.text" | "agent.content" => {
                                        if let Some(text) = payload.and_then(|p| p.get("text")).and_then(|t| t.as_str()) {
                                            content.text = text.to_string();
                                            yield ClientResult::new_ok(content.clone());
                                        }
                                    }
                                    "agent.text.delta" | "agent.content.delta" => {
                                        if let Some(delta) = payload.and_then(|p| p.get("delta")).and_then(|d| d.as_str()) {
                                            content.text.push_str(delta);
                                            yield ClientResult::new_ok(content.clone());
                                        } else if let Some(text) = payload.and_then(|p| p.get("text")).and_then(|t| t.as_str()) {
                                            content.text.push_str(text);
                                            yield ClientResult::new_ok(content.clone());
                                        }
                                    }
                                    // Agent completion
                                    "agent.done" | "agent.complete" | "agent.end" => {
                                        yield ClientResult::new_ok(content.clone());
                                        break;
                                    }
                                    // Agent error
                                    "agent.error" => {
                                        let error_msg = payload
                                            .and_then(|p| p.get("message").or(p.get("error")))
                                            .map(|e| e.to_string())
                                            .unwrap_or_else(|| "Unknown error".to_string());
                                        yield ClientError::new(
                                            ClientErrorKind::Response,
                                            format!("OpenClaw error: {}", error_msg),
                                        ).into();
                                        break;
                                    }
                                    // Agent streaming event - extract delta text
                                    "agent" => {
                                        if let Some(data) = payload.and_then(|p| p.get("data")) {
                                            // Use delta for incremental updates
                                            if let Some(delta) = data.get("delta").and_then(|d| d.as_str()) {
                                                content.text.push_str(delta);
                                                yield ClientResult::new_ok(content.clone());
                                            }
                                        }
                                    }
                                    // Chat event with full message
                                    "chat" => {
                                        if let Some(state) = payload.and_then(|p| p.get("state")).and_then(|s| s.as_str()) {
                                            if state == "done" || state == "complete" {
                                                // Final message
                                                yield ClientResult::new_ok(content.clone());
                                                break;
                                            }
                                        }
                                    }
                                    _ => {
                                        log::debug!("OpenClaw ignoring event: {}", event_name);
                                    }
                                }
                            }
                            // Handle responses
                            "res" => {
                                let ok = json.get("ok").and_then(|o| o.as_bool()).unwrap_or(false);

                                if !ok {
                                    let error_msg = json.get("error")
                                        .map(|e| e.to_string())
                                        .unwrap_or_else(|| "Unknown error".to_string());
                                    yield ClientError::new(
                                        ClientErrorKind::Response,
                                        format!("OpenClaw error: {}", error_msg),
                                    ).into();
                                    break;
                                }

                                // Check payload for response type
                                if let Some(payload) = json.get("payload") {
                                    // Check if this is a hello-ok response (connection successful)
                                    if let Some(res_type) = payload.get("type").and_then(|t| t.as_str()) {
                                        if res_type == "hello-ok" && !connected {
                                            connected = true;
                                            log::debug!("OpenClaw connected successfully, sending agent request");

                                            // Now send the agent request
                                            let agent_request = OpenClawRequest::new(
                                                "agent",
                                                AgentParams {
                                                    message: last_message.clone(),
                                                    idempotency_key: Uuid::new_v4().to_string(),
                                                    session_id: None,
                                                    agent_id: Some("main".to_string()),
                                                },
                                            );

                                            let agent_json = match serde_json::to_string(&agent_request) {
                                                Ok(json) => json,
                                                Err(e) => {
                                                    yield ClientError::new(
                                                        ClientErrorKind::Format,
                                                        format!("Failed to serialize agent request: {}", e),
                                                    ).into();
                                                    return;
                                                }
                                            };

                                            log::debug!("OpenClaw sending agent request: {}", agent_json);

                                            if let Err(e) = write.send(WsMessage::Text(agent_json.into())).await {
                                                yield ClientError::new(
                                                    ClientErrorKind::Network,
                                                    format!("Failed to send agent request: {}", e),
                                                ).into();
                                                return;
                                            }
                                        }
                                    }

                                    // Check if this is an agent response with result
                                    if let Some(status) = payload.get("status").and_then(|s| s.as_str()) {
                                        if status == "ok" || status == "completed" {
                                            // Extract text from result.payloads[].text
                                            if let Some(result) = payload.get("result") {
                                                if let Some(payloads) = result.get("payloads").and_then(|p| p.as_array()) {
                                                    for p in payloads {
                                                        if let Some(text) = p.get("text").and_then(|t| t.as_str()) {
                                                            content.text.push_str(text);
                                                        }
                                                    }
                                                    if !content.text.is_empty() {
                                                        yield ClientResult::new_ok(content.clone());
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                log::debug!("OpenClaw ignoring message type: {}", msg_type);
                            }
                        }
                    }
                    Ok(WsMessage::Close(_)) => {
                        log::debug!("OpenClaw WebSocket closed");
                        break;
                    }
                    Ok(_) => {
                        // Ignore other message types (Binary, Ping, Pong)
                    }
                    Err(e) => {
                        yield ClientError::new(
                            ClientErrorKind::Network,
                            format!("WebSocket error: {}", e),
                        ).into();
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
        let inner = self.0.read().unwrap().clone();

        // WebSocket not supported on WASM yet
        let stream = stream! {
            let content = MessageContent {
                text: format!(
                    "OpenClaw Gateway 在 Web 平台暂不支持。\n\n\
                    请使用桌面版 Moly-AI 或直接访问 OpenClaw Web UI: {}",
                    inner.url.replace("ws://", "http://").replace("wss://", "https://")
                ),
                ..Default::default()
            };
            yield ClientResult::new_ok(content);
        };

        Box::pin(stream)
    }
}
