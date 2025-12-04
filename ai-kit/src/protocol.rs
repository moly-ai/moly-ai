//! This module contains the core types and traits used by all AI Kit apps.

// Keeping submodules private to make the protocol look "flat" from outside.
mod attachment;
mod client;
mod entity;
mod message;
mod realtime;
mod tool;

pub use attachment::*;
pub use client::*;
pub use entity::*;
pub use message::*;
pub use realtime::*;
pub use tool::*;

// Re-export relevant, protocol related, async types.
pub use crate::utils::asynchronous::{BoxPlatformSendFuture, BoxPlatformSendStream};
