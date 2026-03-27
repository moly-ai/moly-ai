# Schema Evolution

Moly persists application data as JSON on disk. When the Rust structs that
back this data change shape, old saved files can crash the app for existing
users if the change is not handled carefully.

Serde gives us several tools to deal with this.

---

## Adding New Fields with Defaults

The simplest case: you add a new field to a struct that is already saved on
disk. Old JSON files will not contain it, so deserialization fails unless you
provide a fallback.

Use `#[serde(default)]` to fill the missing field with its `Default` value:

```rust
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    // Added later — old files won't have this field.
    #[serde(default)]
    pub font_size: u32,
}
```

When the type's `Default` is not the right fallback, point to a function:

```rust
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub theme: String,
    #[serde(default = "default_font_size")]
    pub font_size: u32,
}

fn default_font_size() -> u32 {
    14
}
```

## Implementing `Deserialize` Manually

This gives you full control over how a type is deserialized. The most advanced
form uses a serde `Visitor`, but that is rarely necessary. A simpler approach
is to deserialize into an intermediate type — a proxy struct, a primitive, a
`serde_json::Value`, etc. — and produce the final value from there.

```rust
use serde::{Deserialize, Deserializer};

// The clean domain type — does NOT derive Deserialize.
#[derive(Debug)]
pub struct Event {
    pub data: String,
}

// A private proxy that matches the historical JSON shape.
#[derive(Deserialize)]
struct EventProxy {
    old_info: Option<String>,
    new_data: Option<String>,
}

impl<'de> Deserialize<'de> for Event {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let proxy = EventProxy::deserialize(deserializer)?;

        let data = proxy
            .new_data
            .or(proxy.old_info)
            .unwrap_or_else(|| "default".to_string());

        Ok(Event { data })
    }
}
```

### Alternative: `#[serde(from = "Proxy")]`

For this same proxy pattern, serde offers `#[serde(from = "...")]` as a
struct-level attribute. Instead of implementing `Deserialize` by hand, you
implement `From<Proxy> for CleanType` and serde connects them automatically:

```rust
#[derive(Deserialize)]
#[serde(from = "LegacyUser")]
struct User {
    name: String,
    age: u32,
}

#[derive(Deserialize)]
struct LegacyUser {
    name: String,
    age_text: Option<String>,
    age_num: Option<u32>,
}

impl From<LegacyUser> for User {
    fn from(legacy: LegacyUser) -> Self {
        let age = legacy.age_num.unwrap_or_else(|| {
            legacy.age_text.unwrap_or_default().parse().unwrap_or(0)
        });
        User { name: legacy.name, age }
    }
}
```

The conversion logic is plain Rust — no serde internals. The tradeoff is that
the `from` attribute hides the deserialization flow behind macro magic, which
can be harder to trace if you are not familiar with the feature.

## Using `deserialize_with` on Fields

When you need to control how a specific field deserializes — without changing
the `Deserialize` implementation of the field's type — use the
`#[serde(deserialize_with = "...")]` field attribute.

```rust
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct Config {
    #[serde(deserialize_with = "flexible_bool")]
    pub enabled: bool,
}

/// Accepts `true`, `false`, `1`, `0`, `"true"`, `"false"`.
fn flexible_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match &value {
        serde_json::Value::Bool(b) => Ok(*b),
        serde_json::Value::Number(n) => Ok(n.as_u64() == Some(1)),
        serde_json::Value::String(s) => match s.as_str() {
            "true" | "1" => Ok(true),
            "false" | "0" => Ok(false),
            _ => Err(serde::de::Error::custom("expected a boolean-like value")),
        },
        _ => Err(serde::de::Error::custom("expected a boolean-like value")),
    }
}
```

This is useful when the field's type itself (e.g. `bool`, `String`) should not
change behavior globally, but a particular JSON source writes it in a
non-standard way.

## Renaming Fields or Variants with Aliases

When you rename a field or enum variant, old saved data still contains the
previous name. Use `#[serde(alias = "...")]` to accept both the old and new
names during deserialization.

```rust
#[derive(Serialize, Deserialize)]
pub enum ProviderType {
    #[serde(alias = "OpenAI")]
    OpenAi,
    #[serde(alias = "OpenAIRealtime")]
    OpenAiRealtime,
}
```

New data is serialized with the current name. Old data containing the alias
still deserializes correctly. This also works on struct fields:

```rust
#[derive(Serialize, Deserialize)]
pub struct Config {
    #[serde(alias = "api_key")]
    pub key: String,
}
