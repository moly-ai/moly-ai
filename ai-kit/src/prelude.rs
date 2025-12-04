//! Re-exports common abstractions that are likely to be used.

// The most important module in this crate.
pub use crate::protocol::*;

// These are the clients that are most commonly used.
pub use crate::clients::{multi::MultiClient, openai::OpenAIClient};

// If we re-export clients, then we may also re-export tools.
pub use crate::mcp::mcp_manager::{McpManagerClient, McpTransport};

// Only used by users that want the built-in chat business logic. But this is expected.
pub use crate::controllers::chat::*;

// Common mutation types used by controllers.
pub use crate::utils::vec::{IndexSet, VecEffect, VecMutation};
