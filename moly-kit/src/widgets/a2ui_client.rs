//! A2UI-aware BotClient wrapper.
//!
//! When A2UI is enabled, this client prepends an A2UI system prompt
//! describing the A2UI adjacency-list protocol so the LLM generates
//! UI JSON as structured output in its response text.

use crate::aitk::protocol::{
    Bot, BotId, ClientResult, EntityId, Message, MessageContent, Tool,
};
use crate::aitk::protocol::BotClient;
use crate::aitk::utils::asynchronous::{BoxPlatformSendFuture, BoxPlatformSendStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

// ============================================================================
// Global A2UI state
// ============================================================================

/// Global A2UI enabled flag — set from PromptInput, read by A2uiClient.
static GLOBAL_A2UI_ENABLED: AtomicBool = AtomicBool::new(false);

/// Set the global A2UI enabled state.
pub fn set_global_a2ui_enabled(enabled: bool) {
    ::log::info!("[A2UI] Global enabled set to: {}", enabled);
    GLOBAL_A2UI_ENABLED.store(enabled, Ordering::SeqCst);
}

/// Get the global A2UI enabled state.
pub fn is_global_a2ui_enabled() -> bool {
    GLOBAL_A2UI_ENABLED.load(Ordering::SeqCst)
}

/// Pending A2UI JSON — written by Chat widget, read by shell App.
static PENDING_A2UI_JSON: Mutex<Option<String>> = Mutex::new(None);

/// Store pending A2UI JSON for the shell app to render.
pub fn set_pending_a2ui_json(json: String) {
    ::log::info!(
        "[A2UI] Storing pending JSON ({} bytes)",
        json.len()
    );
    *PENDING_A2UI_JSON.lock().unwrap() = Some(json);
}

/// Take pending A2UI JSON (clears the buffer).
pub fn take_pending_a2ui_json() -> Option<String> {
    PENDING_A2UI_JSON.lock().unwrap().take()
}

// ============================================================================
// A2UI JSON extraction
// ============================================================================

/// Extract A2UI JSON from ` ```a2ui ``` ` code fences in the text.
///
/// Returns `(clean_text, Option<a2ui_json>)` where `clean_text` has
/// the code fence block removed, and `a2ui_json` is the extracted JSON
/// string (if any).
///
/// During streaming, call with `force = false` — if no closing fence is
/// found, the JSON is not extracted (still incomplete).
/// After streaming ends, call with `force = true` — if no closing fence
/// is found, treat everything after the opening fence as JSON (the LLM
/// may have omitted the closing fence).
pub fn extract_a2ui_json(text: &str, force: bool) -> (String, Option<String>) {
    let fence_start = "```a2ui";
    let fence_end = "```";

    let Some(start_idx) = text.find(fence_start) else {
        return (text.to_string(), None);
    };

    let json_start = start_idx + fence_start.len();

    // Skip optional whitespace/newline after the opening fence
    let json_start = text[json_start..]
        .find(|c: char| !c.is_whitespace() || c == '[' || c == '{')
        .map(|i| json_start + i)
        .unwrap_or(json_start);

    // Find the closing fence after the JSON content
    let (json_end, block_end) =
        if let Some(end_idx) = text[json_start..].find(fence_end) {
            let json_end = json_start + end_idx;
            (json_end, json_end + fence_end.len())
        } else if force {
            // No closing fence but streaming is done — use rest of text
            (text.len(), text.len())
        } else {
            // No closing fence yet (likely still streaming).
            // Hide everything from the opening fence onward.
            let clean = text[..start_idx].trim().to_string();
            return (clean, None);
        };

    let json_str = text[json_start..json_end].trim().to_string();
    if json_str.is_empty() {
        return (text.to_string(), None);
    }

    // Build cleaned text: everything before the block + everything after
    let mut clean = text[..start_idx].to_string();
    clean.push_str(&text[block_end..]);
    let clean = clean.trim().to_string();

    (clean, Some(json_str))
}

// ============================================================================
// A2UI system prompt
// ============================================================================

/// System prompt describing the full A2UI adjacency-list protocol.
const A2UI_SYSTEM_PROMPT: &str = r#"You can generate interactive UIs using the A2UI protocol.
When the user asks you to create or update a UI, output A2UI JSON wrapped in a code fence:

```a2ui
[ ... ]
```

You may include a brief explanation outside the fence.

IMPORTANT:
- Output valid JSON only inside the code fence. Do NOT add comments (no // or /* */).
- Only use the component types listed below. Do NOT invent new types (no Tabs, Divider, Icon, Avatar, etc.).
- Do NOT use Image components unless you have a real, working https URL. Use Text with emoji or descriptive labels instead.

# Design Guidelines

When the user requests an app, expand their request into a polished, feature-rich UI:
- Flesh out the concept: add logical sections, realistic sample data, and secondary features a real app would have.
- Use **Card** components generously to visually group related content with elevation for depth and structure.
- Organize sections vertically with **Column** as the root. Use header **Text** (h1/h2) to label major sections and **Text** (h4) for card titles.
- Include realistic, varied sample data in the data model (real names, dates, dollar amounts, descriptions — not generic placeholders).
- Use **Row** layouts with weight to create multi-column displays (e.g. label on left, value on right; or icon-emoji on left, content on right).
- Use the full range of **usageHint** values (h1 for page titles, h2 for section headers, h4 for card titles, body for content, caption for secondary/muted text, code for data values).
- Add interactive elements: TextField for search/input, CheckBox for toggles, Slider for adjustable values, Button for actions.
- Use **List** with templates for data-driven repeating items (transactions, messages, contacts, etc.).
- Use emoji characters in Text labels to add visual richness (e.g. "🏦 My Bank", "💰 Balance", "📊 Stats").
- Aim for 30-50 components to create a substantive, app-like experience.

# A2UI Protocol

Output a JSON array with three messages:

1. **beginRendering** — initialize the surface with a root component ID
2. **surfaceUpdate** — define all components as a flat adjacency list
3. **dataModelUpdate** — set initial data values

# Component Types

## Layout
- **Column** — vertical layout
  `{"Column": {"children": {"explicitList": ["id1","id2"]}, "alignment": "center", "distribution": "spaceBetween"}}`
- **Row** — horizontal layout (same fields as Column)
- **Card** — styled container with elevation
  `{"Card": {"child": "content-id", "elevation": 2}}`
- **List** — scrollable data-driven list
  `{"List": {"children": {"template": {"componentId": "item-tpl", "dataBinding": "/items"}}, "direction": "vertical"}}`

## Display
- **Text** — text label
  `{"Text": {"text": {"literalString": "Hello"}, "usageHint": "h1"}}`
  usageHint options: h1, h2, h3, h4, h5, body, caption, code
- **Image** — image display
  `{"Image": {"url": {"literalString": "https://..."}, "fit": "cover", "usageHint": "mediumFeature"}}`

## Interactive
- **Button** — clickable button (child is a text component ID)
  `{"Button": {"child": "btn-label", "primary": true, "action": {"name": "submit", "context": []}}}`
- **TextField** — text input (binds to data model path)
  `{"TextField": {"text": {"path": "/form/name"}, "label": {"literalString": "Name"}, "placeholder": {"literalString": "Enter name"}}}`
- **CheckBox** — toggle (binds to data model path)
  `{"CheckBox": {"value": {"path": "/settings/darkMode"}, "label": {"literalString": "Dark Mode"}}}`
- **Slider** — numeric slider (binds to data model path)
  `{"Slider": {"value": {"path": "/volume"}, "min": 0, "max": 100, "step": 1}}`

# Value Types

Static values:
- `{"literalString": "text"}`, `{"literalNumber": 42}`, `{"literalBoolean": true}`

Data-bound values (two-way binding for interactive controls):
- `{"path": "/data/key"}`

# Data Model Values

In `dataModelUpdate.contents`, each entry has a `key` and one value field:
- `{"key": "name", "valueString": "Alice"}`
- `{"key": "count", "valueNumber": 0}`
- `{"key": "enabled", "valueBoolean": true}`
- `{"key": "items", "valueArray": [...]}`
- `{"key": "user", "valueMap": [{"key": "name", "valueString": "..."}]}`

# Children

- **explicitList**: fixed children: `{"explicitList": ["child1", "child2"]}`
- **template**: data-driven list: `{"template": {"componentId": "tpl-id", "dataBinding": "/items"}}`

# Component Definition

Each component in `surfaceUpdate.components`:
```json
{"id": "unique-id", "component": {"Text": {...}}, "weight": 1.0}
```
`weight` is optional (used for flex sizing in Row/Column).

# Complete Example

User: "Create a counter app"

```a2ui
[
  {"beginRendering": {"surfaceId": "main", "root": "root"}},
  {"surfaceUpdate": {"surfaceId": "main", "components": [
    {"id": "root", "component": {"Column": {"children": {"explicitList": ["title", "count-display", "buttons"]}, "alignment": "center"}}},
    {"id": "title", "component": {"Text": {"text": {"literalString": "Counter"}, "usageHint": "h1"}}},
    {"id": "count-display", "component": {"Text": {"text": {"path": "/count"}, "usageHint": "h2"}}},
    {"id": "buttons", "component": {"Row": {"children": {"explicitList": ["dec-btn", "inc-btn"]}, "distribution": "spaceEvenly"}}},
    {"id": "dec-label", "component": {"Text": {"text": {"literalString": "-"}}}},
    {"id": "dec-btn", "component": {"Button": {"child": "dec-label", "action": {"name": "decrement", "context": []}}}},
    {"id": "inc-label", "component": {"Text": {"text": {"literalString": "+"}}}},
    {"id": "inc-btn", "component": {"Button": {"child": "inc-label", "primary": true, "action": {"name": "increment", "context": []}}}}
  ]}},
  {"dataModelUpdate": {"surfaceId": "main", "path": "/", "contents": [
    {"key": "count", "valueNumber": 0}
  ]}}
]
```
"#;

// ============================================================================
// A2uiClient
// ============================================================================

/// A wrapper around a [`BotClient`] that injects the A2UI system prompt
/// when A2UI mode is enabled, so the LLM generates A2UI JSON as
/// structured output in its response text.
pub struct A2uiClient {
    client: Box<dyn BotClient>,
    a2ui_enabled: Arc<AtomicBool>,
}

impl Clone for A2uiClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone_box(),
            a2ui_enabled: self.a2ui_enabled.clone(),
        }
    }
}

impl A2uiClient {
    /// Create a new A2UI-aware client wrapper.
    pub fn new(client: Box<dyn BotClient>) -> Self {
        Self {
            client,
            a2ui_enabled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Enable or disable A2UI mode.
    pub fn set_a2ui_enabled(&self, enabled: bool) {
        self.a2ui_enabled.store(enabled, Ordering::SeqCst);
    }

    /// Check if A2UI is currently enabled.
    pub fn is_a2ui_enabled(&self) -> bool {
        self.a2ui_enabled.load(Ordering::SeqCst)
    }
}

impl BotClient for A2uiClient {
    fn bots(
        &mut self,
    ) -> BoxPlatformSendFuture<'static, ClientResult<Vec<Bot>>> {
        self.client.bots()
    }

    fn clone_box(&self) -> Box<dyn BotClient> {
        Box::new(self.clone())
    }

    fn send(
        &mut self,
        bot_id: &BotId,
        messages: &[Message],
        tools: &[Tool],
    ) -> BoxPlatformSendStream<'static, ClientResult<MessageContent>> {
        let instance_enabled =
            self.a2ui_enabled.load(Ordering::SeqCst);
        let global_enabled = is_global_a2ui_enabled();
        let a2ui_enabled = instance_enabled || global_enabled;

        if !a2ui_enabled {
            eprintln!("[A2UI send] disabled (instance={}, global={})", instance_enabled, global_enabled);
            return self.client.send(bot_id, messages, tools);
        }

        eprintln!(
            "[A2UI send] Enabled — prepending system prompt ({} messages)",
            messages.len()
        );

        // Prepend A2UI system prompt, then forward to wrapped client
        let mut all_messages = vec![Message {
            from: EntityId::System,
            content: MessageContent {
                text: A2UI_SYSTEM_PROMPT.to_string(),
                ..Default::default()
            },
            ..Default::default()
        }];
        all_messages.extend(messages.to_vec());

        self.client.send(bot_id, &all_messages, tools)
    }
}
