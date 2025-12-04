use super::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Standard message content format.
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct MessageContent {
    /// The main body/document of this message.
    ///
    /// This would normally be written in somekind of document format like
    /// markdown, html, plain text, etc. Only markdown is expected by default.
    pub text: String,

    /// List of citations/sources (urls) associated with this message.
    pub citations: Vec<String>,

    /// The reasoning/thinking content of this message.
    #[cfg_attr(
        feature = "json",
        serde(deserialize_with = "crate::utils::serde::deserialize_default_on_error")
    )]
    pub reasoning: String,

    /// File attachments in this content.
    #[cfg_attr(feature = "json", serde(default))]
    pub attachments: Vec<Attachment>,

    /// Tool calls made by the AI (for assistant messages)
    #[cfg_attr(feature = "json", serde(default))]
    pub tool_calls: Vec<ToolCall>,

    /// Tool call results (for tool messages)
    #[cfg_attr(feature = "json", serde(default))]
    pub tool_results: Vec<ToolResult>,

    /// Non-standard data contained by this message.
    ///
    /// May be used by clients for tracking purposes or to represent unsupported
    /// content.
    ///
    /// This is not expected to be used by most clients.
    // TODO: Using `String` for now because:
    //
    // - `Box<dyn Trait>` can't be `Deserialize`.
    // - `serde_json::Value` would force `serde_json` usage.
    // - `Vec<u8>` has unefficient serialization format and doesn't have many
    //   advantages over `String`.
    //
    // A wrapper type over Value and Box exposing a unified interface could be
    // a solution for later.
    pub data: Option<String>,

    /// Optional upgrade to realtime communication
    #[cfg_attr(feature = "json", serde(skip))]
    pub upgrade: Option<Upgrade>,
}

impl MessageContent {
    /// Checks if the content is absolutely empty (contains no data at all).
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
            && self.citations.is_empty()
            && self.data.is_none()
            && self.reasoning.is_empty()
            && self.attachments.is_empty()
            && self.tool_calls.is_empty()
            && self.tool_results.is_empty()
            && self.upgrade.is_none()
    }
}

/// Metadata automatically tracked by MolyKit for each message.
///
/// "Metadata" basically means "data about data". Like tracking timestamps for
/// data modification.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct MessageMetadata {
    /// Runtime flag indicating that the message is still incomplete (being written).
    ///
    /// Skipped during serialization.
    #[cfg_attr(feature = "json", serde(skip))]
    pub is_writing: bool,

    /// When the message got created.
    ///
    /// Default to epoch if missing during deserialization. Otherwise, if constructed
    /// by [`MessageMetadata::default`], it defaults to "now".
    #[cfg_attr(feature = "json", serde(default))]
    pub created_at: DateTime<Utc>,

    /// Last time the reasoning/thinking content was updated.
    ///
    /// Default to epoch if missing during deserialization. Otherwise, if constructed
    /// by [`MessageMetadata::default`], it defaults to "now".
    #[cfg_attr(feature = "json", serde(default))]
    pub reasoning_updated_at: DateTime<Utc>,

    /// Last time the main text was updated.
    ///
    /// Default to epoch if missing during deserialization. Otherwise, if constructed
    /// by [`MessageMetadata::default`], it defaults to "now".
    #[cfg_attr(feature = "json", serde(default))]
    pub text_updated_at: DateTime<Utc>,
}

impl Default for MessageMetadata {
    fn default() -> Self {
        // Use the same timestamp for all fields.
        let now = Utc::now();
        MessageMetadata {
            is_writing: false,
            created_at: now,
            reasoning_updated_at: now,
            text_updated_at: now,
        }
    }
}

impl MessageMetadata {
    /// Same behavior as [`MessageMetadata::default`].
    pub fn new() -> Self {
        MessageMetadata::default()
    }

    /// Create a new metadata with all fields set to default but timestamps set to epoch.
    pub fn epoch() -> Self {
        MessageMetadata {
            is_writing: false,
            created_at: DateTime::UNIX_EPOCH,
            reasoning_updated_at: DateTime::UNIX_EPOCH,
            text_updated_at: DateTime::UNIX_EPOCH,
        }
    }
}

impl MessageMetadata {
    /// The inferred amount of time the reasoning step took, in seconds (with milliseconds).
    pub fn reasoning_time_taken_seconds(&self) -> f32 {
        let delta = self.reasoning_updated_at - self.created_at;
        delta.as_seconds_f32()
    }

    pub fn is_idle(&self) -> bool {
        !self.is_writing
    }

    pub fn is_writing(&self) -> bool {
        self.is_writing
    }
}

/// A message that is part of a conversation.
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct Message {
    /// The id of who sent this message.
    pub from: EntityId,

    /// Auto-generated metadata for this message.
    ///
    /// If missing during deserialization, uses [`MessageMetadata::epoch`] instead
    /// of [`MessageMetadata::default`].
    #[cfg_attr(feature = "json", serde(default = "MessageMetadata::epoch"))]
    pub metadata: MessageMetadata,

    /// The parsed content of this message ready to present.
    pub content: MessageContent,
}

impl Message {
    /// Shorthand for constructing an app error message.
    pub fn app_error(error: impl fmt::Display) -> Self {
        Message {
            from: EntityId::App,
            content: MessageContent {
                text: format!("Error: {}", error),
                ..MessageContent::default()
            },
            ..Default::default()
        }
    }

    /// Set the content of a message as a whole (also updates metadata).
    pub fn set_content(&mut self, content: MessageContent) {
        self.update_content(|c| {
            *c = content;
        });
    }

    /// Update specific parts of the content of a message (also updates metadata).
    pub fn update_content(&mut self, f: impl FnOnce(&mut MessageContent)) {
        let bk = self.content.clone();
        let now = Utc::now();

        f(&mut self.content);

        if self.content.text != bk.text {
            self.metadata.text_updated_at = now;
        }

        if self.content.reasoning != bk.reasoning {
            self.metadata.reasoning_updated_at = now;
        }
    }
}
