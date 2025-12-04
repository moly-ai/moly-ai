#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Tool {
    pub name: String,
    pub description: Option<String>,
    /// JSON Schema object defining the expected parameters for the tool
    #[cfg_attr(feature = "json", serde(default))]
    pub input_schema: std::sync::Arc<serde_json::Map<String, serde_json::Value>>,
}

impl Tool {
    pub fn new(name: String, description: Option<String>) -> Self {
        use serde_json::Map;
        use std::sync::Arc;

        Tool {
            name,
            description,
            input_schema: Arc::new(Map::new()),
        }
    }
}

// Conversion traits for rmcp interop on native platforms
#[cfg(not(target_arch = "wasm32"))]
impl From<rmcp::model::Tool> for Tool {
    fn from(rmcp_tool: rmcp::model::Tool) -> Self {
        Tool {
            name: rmcp_tool.name.into_owned(),
            description: rmcp_tool.description.map(|d| d.into_owned()),
            input_schema: rmcp_tool.input_schema,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl From<Tool> for rmcp::model::Tool {
    fn from(tool: Tool) -> Self {
        rmcp::model::Tool {
            name: tool.name.into(),
            description: tool.description.map(|d| d.into()),
            input_schema: tool.input_schema,
            output_schema: None,
            annotations: None,
        }
    }
}

/// Permission status for tool call execution
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum ToolCallPermissionStatus {
    /// Waiting for user decision
    #[default]
    Pending,
    /// User approved execution
    Approved,
    /// User denied execution
    Denied,
}

/// Represents a function/tool call made by the AI
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ToolCall {
    /// Unique identifier for this tool call
    pub id: String,
    /// Name of the tool/function to call
    pub name: String,
    /// Arguments passed to the tool (JSON)
    pub arguments: serde_json::Map<String, serde_json::Value>,
    /// Permission status for this tool call
    #[cfg_attr(feature = "json", serde(default))]
    pub permission_status: ToolCallPermissionStatus,
}

/// Represents the result of a tool call execution
#[derive(Clone, PartialEq, Debug)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct ToolResult {
    /// The tool call ID this result corresponds to
    pub tool_call_id: String,
    /// The result content from the tool execution
    pub content: String,
    /// Whether the tool call was successful
    pub is_error: bool,
}
