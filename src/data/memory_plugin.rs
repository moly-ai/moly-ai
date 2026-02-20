use moly_kit::aitk::{
    controllers::chat::ChatControllerPlugin,
    protocol::*,
};
use serde_json::{Map, Value};
use std::sync::Arc;

use super::memory::MemoryStore;

const TOOL_SAVE: &str = "memory__save";
const TOOL_DELETE: &str = "memory__delete";
const TOOL_LIST: &str = "memory__list";

/// Plugin that provides persistent user memory across conversations.
///
/// Injects remembered facts into the system prompt and exposes tools
/// for the model to create, delete, and list memories.
pub struct MemoryPlugin {
    store: MemoryStore,
}

impl MemoryPlugin {
    /// Creates a new memory plugin with the given store.
    pub fn new(store: MemoryStore) -> Self {
        Self { store }
    }

    fn build_system_message(&self) -> Message {
        let memories_text = self.store.format_for_prompt();

        let text = if memories_text.is_empty() {
            "You can save facts about the user to persistent memory using the \
             memory__save tool. Saved memories are automatically recalled in \
             future conversations. When the user shares preferences, personal \
             info, or important context, save it. Do not mention the memory \
             system, tools, or how it works to the user unless they ask."
                .to_string()
        } else {
            format!(
                "Below are facts you've previously saved about the user. Use \
                 them to personalize responses. Save new facts when the user \
                 shares preferences, personal info, or important context using \
                 the memory__save tool. Do not mention the memory system, \
                 tools, or how it works to the user unless they ask.\n\n\
                 ## Memories\n{memories_text}"
            )
        };

        Message {
            from: EntityId::System,
            content: MessageContent {
                text,
                data: Some("__memory_system__".to_string()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn build_tool_schema(
        properties: &[(&str, &str, &str)],
        required: &[&str],
    ) -> Arc<Map<String, Value>> {
        let mut schema = Map::new();
        schema.insert("type".to_string(), Value::String("object".to_string()));

        let mut props = Map::new();
        for (name, type_str, description) in properties {
            let mut prop = Map::new();
            prop.insert("type".to_string(), Value::String(type_str.to_string()));
            prop.insert(
                "description".to_string(),
                Value::String(description.to_string()),
            );
            props.insert(name.to_string(), Value::Object(prop));
        }
        schema.insert("properties".to_string(), Value::Object(props));

        let required: Vec<Value> = required
            .iter()
            .map(|r| Value::String(r.to_string()))
            .collect();
        schema.insert("required".to_string(), Value::Array(required));

        Arc::new(schema)
    }
}

impl ChatControllerPlugin for MemoryPlugin {
    fn on_send_messages(&self, messages: &mut Vec<Message>) {
        let system_msg = self.build_system_message();

        // Update existing memory system message or insert at the front.
        if let Some(index) = messages.iter().position(is_memory_system_message) {
            messages[index] = system_msg;
        } else {
            messages.insert(0, system_msg);
        }
    }

    fn tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: TOOL_SAVE.to_string(),
                description: Some(
                    "Save a fact about the user to persistent memory. Use this \
                     when the user shares preferences, personal information, or \
                     important context worth remembering across conversations."
                        .to_string(),
                ),
                input_schema: Self::build_tool_schema(
                    &[("content", "string", "The fact to remember about the user")],
                    &["content"],
                ),
                auto_execute: true,
                silent: true,
            },
            Tool {
                name: TOOL_DELETE.to_string(),
                description: Some(
                    "Delete a previously saved memory by its ID."
                        .to_string(),
                ),
                input_schema: Self::build_tool_schema(
                    &[("id", "string", "The ID of the memory to delete")],
                    &["id"],
                ),
                auto_execute: true,
                silent: true,
            },
            Tool {
                name: TOOL_LIST.to_string(),
                description: Some(
                    "List all saved memories about the user."
                        .to_string(),
                ),
                input_schema: Self::build_tool_schema(&[], &[]),
                auto_execute: true,
                silent: true,
            },
        ]
    }

    fn execute_tool_call(
        &mut self,
        tool_call: &ToolCall,
    ) -> Option<BoxPlatformSendFuture<'static, ToolResult>> {
        let tool_call_id = tool_call.id.clone();

        match tool_call.name.as_str() {
            TOOL_SAVE => {
                let content = tool_call
                    .arguments
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                if content.is_empty() {
                    return Some(Box::pin(async move {
                        ToolResult {
                            tool_call_id,
                            content: "Error: content is required."
                                .to_string(),
                            is_error: true,
                        }
                    }));
                }

                let memory = self.store.add(content);
                let result_text =
                    format!("Memory saved (id: {}).", memory.id);

                Some(Box::pin(async move {
                    ToolResult {
                        tool_call_id,
                        content: result_text,
                        is_error: false,
                    }
                }))
            }
            TOOL_DELETE => {
                let id = tool_call
                    .arguments
                    .get("id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();

                let removed = self.store.remove(&id);
                let result_text = if removed {
                    "Memory deleted.".to_string()
                } else {
                    format!("No memory found with id '{id}'.")
                };

                Some(Box::pin(async move {
                    ToolResult {
                        tool_call_id,
                        content: result_text,
                        is_error: !removed,
                    }
                }))
            }
            TOOL_LIST => {
                let memories = self.store.memories();
                let text = if memories.is_empty() {
                    "No memories saved yet.".to_string()
                } else {
                    memories
                        .iter()
                        .map(|m| format!("- [{}] {}", m.id, m.content))
                        .collect::<Vec<_>>()
                        .join("\n")
                };

                Some(Box::pin(async move {
                    ToolResult {
                        tool_call_id,
                        content: text,
                        is_error: false,
                    }
                }))
            }
            _ => None,
        }
    }
}

/// Identifies the memory system message by its data field.
fn is_memory_system_message(message: &Message) -> bool {
    message.from == EntityId::System
        && message.content.data.as_deref() == Some("__memory_system__")
}
