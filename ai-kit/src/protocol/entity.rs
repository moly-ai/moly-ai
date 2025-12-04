use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

#[cfg(feature = "json")]
use serde::{Deserialize, Serialize};

/// The picture/avatar of an entity that may be represented/encoded in different ways.
// TODO: Consider Arc<str> where applicable.
#[derive(Clone, Debug, PartialEq)]
pub enum Picture {
    /// Normally, one or two graphemes representing the entity.
    Text(String),
    /// An image located at the given path/URL.
    Image(String),
}

impl Picture {
    /// Utility to construct a [`Picture::Text`] from a single grapheme.
    ///
    /// Extracted using unicode segmentation.
    pub fn from_first_grapheme(text: &str) -> Option<Self> {
        use unicode_segmentation::UnicodeSegmentation;
        text.graphemes(true)
            .next()
            .map(|g| g.to_string())
            .map(Picture::Text)
    }
}

/// Indentify the entities that are recognized by this crate, mainly in a chat.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum EntityId {
    /// Represents the user operating this app.
    User,

    /// Represents the `system`/`developer` expected by many LLMs in the chat
    /// context to customize the chat experience and behavior.
    System,

    /// Represents a bot, which is an automated assistant of any kind (model, agent, etc).
    Bot(BotId),

    /// Represents tool execution results and tool-related system messages.
    /// Maps to the "tool" role in LLM APIs.
    Tool,

    /// This app itself. Normally appears when app specific information must be displayed
    /// (like inline errors).
    ///
    /// It's not supposed to be sent as part of a conversation to bots.
    #[default]
    App,
}

/// Represents the capabilities of a bot
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub enum BotCapability {
    /// Bot supports realtime audio communication
    Realtime,
    /// Bot supports image/file attachments
    Attachments,
    /// Bot supports function calling
    FunctionCalling,
}

/// Set of capabilities that a bot supports
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct BotCapabilities {
    capabilities: HashSet<BotCapability>,
}

impl BotCapabilities {
    pub fn new() -> Self {
        Self {
            capabilities: HashSet::new(),
        }
    }

    pub fn with_capability(mut self, capability: BotCapability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    pub fn add_capability(&mut self, capability: BotCapability) {
        self.capabilities.insert(capability);
    }

    pub fn has_capability(&self, capability: &BotCapability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn supports_realtime(&self) -> bool {
        self.has_capability(&BotCapability::Realtime)
    }

    pub fn supports_attachments(&self) -> bool {
        self.has_capability(&BotCapability::Attachments)
    }

    pub fn supports_function_calling(&self) -> bool {
        self.has_capability(&BotCapability::FunctionCalling)
    }

    pub fn iter(&self) -> impl Iterator<Item = &BotCapability> {
        self.capabilities.iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bot {
    /// Unique internal identifier for the bot across all providers
    pub id: BotId,
    pub name: String,
    pub avatar: Picture,
    pub capabilities: BotCapabilities,
}

/// Identifies any kind of bot, local or remote, model or agent, whatever.
///
/// It MUST be globally unique and stable. It should be generated from a provider
/// local id and the domain or url of that provider.
///
/// For serialization, this is encoded as a single string.
#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
#[cfg_attr(feature = "json", derive(Serialize, Deserialize))]
pub struct BotId(Arc<str>);

impl BotId {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Creates a new bot id from a provider local id and a provider domain or url.
    pub fn new(id: &str, provider: &str) -> Self {
        // The id is encoded as: <id_len>;<id>@<provider>.
        // `@` is simply a semantic separator, meaning (literally) "at".
        // The length is what is actually used for separating components allowing
        // these to include `@` characters.
        let id = format!("{};{}@{}", id.len(), id, provider);
        BotId(id.into())
    }

    fn deconstruct(&self) -> (usize, &str) {
        let (id_length, raw) = self.0.split_once(';').expect("malformed bot id");
        let id_length = id_length.parse::<usize>().expect("malformed bot id");
        (id_length, raw)
    }

    /// The id of the bot as it is known by its provider.
    ///
    /// This may not be globally unique.
    pub fn id(&self) -> &str {
        let (id_length, raw) = self.deconstruct();
        &raw[..id_length]
    }

    /// The provider component of this bot id.
    pub fn provider(&self) -> &str {
        let (id_length, raw) = self.deconstruct();
        // + 1 skips the semantic `@` separator
        &raw[id_length + 1..]
    }
}

impl fmt::Display for BotId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bot_id() {
        // Simple
        let id = BotId::new("123", "example.com");
        assert_eq!(id.as_str(), "3;123@example.com");
        assert_eq!(id.id(), "123");
        assert_eq!(id.provider(), "example.com");

        // Dirty
        let id = BotId::new("a;b@c", "https://ex@a@m;ple.co@m");
        assert_eq!(id.as_str(), "5;a;b@c@https://ex@a@m;ple.co@m");
        assert_eq!(id.id(), "a;b@c");
        assert_eq!(id.provider(), "https://ex@a@m;ple.co@m");

        // Similar yet different
        let id1 = BotId::new("a@", "b");
        let id2 = BotId::new("a", "@b");
        assert_ne!(id1.as_str(), id2.as_str());
        assert_ne!(id1, id2);
    }
}
