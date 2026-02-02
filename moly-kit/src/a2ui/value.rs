//! A2UI Value Types
//!
//! Represents the primitive value types used in A2UI protocol for data binding.

use serde::{Deserialize, Serialize};

/// A string value that can be either a literal or a data-bound path.
///
/// # Examples
///
/// ```json
/// {"literalString": "Hello World"}
/// {"path": "/user/name"}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringValue {
    /// A literal string value
    Literal {
        #[serde(rename = "literalString")]
        literal_string: String,
    },
    /// A path reference to the data model
    Path {
        path: String,
    },
}

impl StringValue {
    /// Create a new literal string value
    pub fn literal(s: impl Into<String>) -> Self {
        StringValue::Literal {
            literal_string: s.into(),
        }
    }

    /// Create a new path reference
    pub fn path(p: impl Into<String>) -> Self {
        StringValue::Path { path: p.into() }
    }

    /// Check if this is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self, StringValue::Literal { .. })
    }

    /// Check if this is a path reference
    pub fn is_path(&self) -> bool {
        matches!(self, StringValue::Path { .. })
    }

    /// Get the path if this is a path reference
    pub fn as_path(&self) -> Option<&str> {
        match self {
            StringValue::Path { path } => Some(path),
            _ => None,
        }
    }

    /// Get the literal string if this is a literal value
    pub fn as_literal(&self) -> Option<&str> {
        match self {
            StringValue::Literal { literal_string } => Some(literal_string),
            _ => None,
        }
    }
}

impl Default for StringValue {
    fn default() -> Self {
        StringValue::literal("")
    }
}

/// A number value that can be either a literal or a data-bound path.
///
/// # Examples
///
/// ```json
/// {"literalNumber": 42}
/// {"path": "/count"}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValue {
    /// A literal number value
    Literal {
        #[serde(rename = "literalNumber")]
        literal_number: f64,
    },
    /// A path reference to the data model
    Path {
        path: String,
    },
}

impl NumberValue {
    /// Create a new literal number value
    pub fn literal(n: f64) -> Self {
        NumberValue::Literal { literal_number: n }
    }

    /// Create a new path reference
    pub fn path(p: impl Into<String>) -> Self {
        NumberValue::Path { path: p.into() }
    }

    /// Check if this is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self, NumberValue::Literal { .. })
    }

    /// Get the path if this is a path reference
    pub fn as_path(&self) -> Option<&str> {
        match self {
            NumberValue::Path { path } => Some(path),
            _ => None,
        }
    }

    /// Get the literal number if this is a literal value
    pub fn as_literal(&self) -> Option<f64> {
        match self {
            NumberValue::Literal { literal_number } => Some(*literal_number),
            _ => None,
        }
    }
}

impl Default for NumberValue {
    fn default() -> Self {
        NumberValue::literal(0.0)
    }
}

/// A boolean value that can be either a literal or a data-bound path.
///
/// # Examples
///
/// ```json
/// {"literalBoolean": true}
/// {"path": "/enabled"}
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BooleanValue {
    /// A literal boolean value
    Literal {
        #[serde(rename = "literalBoolean")]
        literal_boolean: bool,
    },
    /// A path reference to the data model
    Path {
        path: String,
    },
}

impl BooleanValue {
    /// Create a new literal boolean value
    pub fn literal(b: bool) -> Self {
        BooleanValue::Literal { literal_boolean: b }
    }

    /// Create a new path reference
    pub fn path(p: impl Into<String>) -> Self {
        BooleanValue::Path { path: p.into() }
    }

    /// Check if this is a literal value
    pub fn is_literal(&self) -> bool {
        matches!(self, BooleanValue::Literal { .. })
    }

    /// Get the path if this is a path reference
    pub fn as_path(&self) -> Option<&str> {
        match self {
            BooleanValue::Path { path } => Some(path),
            _ => None,
        }
    }

    /// Get the literal boolean if this is a literal value
    pub fn as_literal(&self) -> Option<bool> {
        match self {
            BooleanValue::Literal { literal_boolean } => Some(*literal_boolean),
            _ => None,
        }
    }
}

impl Default for BooleanValue {
    fn default() -> Self {
        BooleanValue::literal(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_value_literal() {
        let json = r#"{"literalString": "Hello"}"#;
        let value: StringValue = serde_json::from_str(json).unwrap();
        assert!(value.is_literal());
        assert_eq!(value.as_literal(), Some("Hello"));
    }

    #[test]
    fn test_string_value_path() {
        let json = r#"{"path": "/user/name"}"#;
        let value: StringValue = serde_json::from_str(json).unwrap();
        assert!(value.is_path());
        assert_eq!(value.as_path(), Some("/user/name"));
    }

    #[test]
    fn test_number_value_literal() {
        let json = r#"{"literalNumber": 42}"#;
        let value: NumberValue = serde_json::from_str(json).unwrap();
        assert!(value.is_literal());
        assert_eq!(value.as_literal(), Some(42.0));
    }

    #[test]
    fn test_boolean_value_literal() {
        let json = r#"{"literalBoolean": true}"#;
        let value: BooleanValue = serde_json::from_str(json).unwrap();
        assert!(value.is_literal());
        assert_eq!(value.as_literal(), Some(true));
    }
}
