//! A2UI Protocol Implementation for Makepad
//!
//! A2UI (Agent-to-UI) is a declarative JSON protocol for AI agents to generate
//! rich, interactive UIs. This module implements the A2UI renderer for Makepad.
//!
//! # Architecture
//!
//! ```text
//! A2UI JSON Messages
//!        ↓
//! A2uiMessageProcessor
//!        ↓
//! ┌──────┴──────┐
//! │             │
//! DataModel  ComponentTree
//!    │             │
//!    └──────┬──────┘
//!           ↓
//!    A2uiSurface (Widget)
//!           ↓
//!    Makepad Native Widgets
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! use makepad_component::a2ui::*;
//!
//! // Parse A2UI JSON message
//! let json = r#"{"beginRendering": {"surfaceId": "main", "root": "root"}}"#;
//! let message: A2uiMessage = serde_json::from_str(json)?;
//!
//! // Process message
//! processor.process_message(message);
//! ```

mod message;
mod data_model;
mod processor;
mod registry;
mod surface;
mod value;
mod sse;
mod a2a_client;
mod host;

pub use message::*;
pub use data_model::*;
pub use processor::*;
pub use registry::*;
pub use surface::*;
pub use value::*;
pub use sse::*;
pub use a2a_client::*;
pub use host::*;

use makepad_widgets::Cx;

/// Initialize A2UI live design components
pub fn live_design(cx: &mut Cx) {
    surface::live_design(cx);
}
