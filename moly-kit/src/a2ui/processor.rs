//! A2UI Message Processor
//!
//! Processes incoming A2UI messages and updates the component tree and data model.

use std::collections::HashMap;

use super::{
    data_model::{DataModel, SurfaceDataModels},
    message::*,
    registry::ComponentRegistry,
    value::{BooleanValue, NumberValue, StringValue},
};

/// Represents a UI surface with its component tree and configuration.
#[derive(Debug, Clone)]
pub struct Surface {
    /// Surface ID
    pub id: String,

    /// Root component ID
    pub root: String,

    /// Style configuration
    pub styles: Option<SurfaceStyles>,

    /// Component definitions by ID
    pub components: HashMap<String, ComponentDefinition>,

    /// Whether the surface needs to be redrawn
    pub needs_redraw: bool,
}

impl Surface {
    /// Create a new surface
    pub fn new(id: String, root: String, styles: Option<SurfaceStyles>) -> Self {
        Surface {
            id,
            root,
            styles,
            components: HashMap::new(),
            needs_redraw: true,
        }
    }

    /// Get a component by ID
    pub fn get_component(&self, id: &str) -> Option<&ComponentDefinition> {
        self.components.get(id)
    }

    /// Get all component IDs
    pub fn component_ids(&self) -> impl Iterator<Item = &String> {
        self.components.keys()
    }

    /// Mark the surface as needing redraw
    pub fn mark_dirty(&mut self) {
        self.needs_redraw = true;
    }

    /// Clear the dirty flag
    pub fn clear_dirty(&mut self) {
        self.needs_redraw = false;
    }
}

/// Event emitted when a surface is created
#[derive(Debug, Clone)]
pub struct SurfaceCreatedEvent {
    pub surface_id: String,
}

/// Event emitted when a surface is updated
#[derive(Debug, Clone)]
pub struct SurfaceUpdatedEvent {
    pub surface_id: String,
    pub updated_components: Vec<String>,
}

/// Event emitted when a surface is deleted
#[derive(Debug, Clone)]
pub struct SurfaceDeletedEvent {
    pub surface_id: String,
}

/// Event emitted when data model is updated
#[derive(Debug, Clone)]
pub struct DataModelUpdatedEvent {
    pub surface_id: String,
    pub updated_paths: Vec<String>,
}

/// Events that can be emitted by the processor
#[derive(Debug, Clone)]
pub enum ProcessorEvent {
    SurfaceCreated(SurfaceCreatedEvent),
    SurfaceUpdated(SurfaceUpdatedEvent),
    SurfaceDeleted(SurfaceDeletedEvent),
    DataModelUpdated(DataModelUpdatedEvent),
}

/// The A2UI message processor.
///
/// Manages surfaces, component trees, and data models.
/// Processes incoming A2UI messages and emits events for UI updates.
///
/// # Example
///
/// ```rust,ignore
/// let registry = ComponentRegistry::with_standard_catalog();
/// let mut processor = A2uiMessageProcessor::new(registry);
///
/// // Process a message
/// let json = r#"{"beginRendering": {"surfaceId": "main", "root": "root"}}"#;
/// let message: A2uiMessage = serde_json::from_str(json)?;
/// let events = processor.process_message(message);
///
/// // Handle events
/// for event in events {
///     match event {
///         ProcessorEvent::SurfaceCreated(e) => {
///             println!("Surface created: {}", e.surface_id);
///         }
///         // ...
///     }
/// }
/// ```
#[derive(Debug)]
pub struct A2uiMessageProcessor {
    /// Component registry
    registry: ComponentRegistry,

    /// Active surfaces by ID
    surfaces: HashMap<String, Surface>,

    /// Data models for each surface
    data_models: SurfaceDataModels,

    /// Pending user actions to send
    pending_actions: Vec<UserAction>,
}

impl A2uiMessageProcessor {
    /// Create a new processor with the given component registry
    pub fn new(registry: ComponentRegistry) -> Self {
        A2uiMessageProcessor {
            registry,
            surfaces: HashMap::new(),
            data_models: SurfaceDataModels::new(),
            pending_actions: Vec::new(),
        }
    }

    /// Create a new processor with the standard component catalog
    pub fn with_standard_catalog() -> Self {
        Self::new(ComponentRegistry::with_standard_catalog())
    }

    /// Get the component registry
    pub fn registry(&self) -> &ComponentRegistry {
        &self.registry
    }

    /// Get a surface by ID
    pub fn get_surface(&self, surface_id: &str) -> Option<&Surface> {
        self.surfaces.get(surface_id)
    }

    /// Get a mutable surface by ID
    pub fn get_surface_mut(&mut self, surface_id: &str) -> Option<&mut Surface> {
        self.surfaces.get_mut(surface_id)
    }

    /// Get all surface IDs
    pub fn surface_ids(&self) -> impl Iterator<Item = &String> {
        self.surfaces.keys()
    }

    /// Get the data model for a surface
    pub fn get_data_model(&self, surface_id: &str) -> Option<&DataModel> {
        self.data_models.get(surface_id)
    }

    /// Get a mutable data model for a surface
    pub fn get_data_model_mut(&mut self, surface_id: &str) -> Option<&mut DataModel> {
        self.data_models.get_mut(surface_id)
    }

    /// Process a single A2UI message
    ///
    /// Returns a list of events that occurred as a result of processing.
    pub fn process_message(&mut self, message: A2uiMessage) -> Vec<ProcessorEvent> {
        match message {
            A2uiMessage::BeginRendering(msg) => self.process_begin_rendering(msg),
            A2uiMessage::SurfaceUpdate(msg) => self.process_surface_update(msg),
            A2uiMessage::DataModelUpdate(msg) => self.process_data_model_update(msg),
            A2uiMessage::DeleteSurface(msg) => self.process_delete_surface(msg),
            A2uiMessage::UserAction(msg) => {
                // UserAction is typically sent TO the server, not processed here
                // But we store it for the host to retrieve
                self.pending_actions.push(msg);
                vec![]
            }
        }
    }

    /// Process multiple A2UI messages (e.g., from a JSON array)
    pub fn process_messages(&mut self, messages: Vec<A2uiMessage>) -> Vec<ProcessorEvent> {
        let mut events = Vec::new();
        for message in messages {
            events.extend(self.process_message(message));
        }
        events
    }

    /// Parse and process a JSON string containing A2UI messages.
    ///
    /// Tries strict array parse first. On failure, falls back to
    /// parsing each element individually (skipping malformed ones)
    /// so that valid messages like `beginRendering` and `surfaceUpdate`
    /// still render even if `dataModelUpdate` has schema issues.
    pub fn process_json(&mut self, json: &str) -> Result<Vec<ProcessorEvent>, serde_json::Error> {
        // Try parsing, repair truncated JSON if needed
        let json = &Self::repair_json(json);

        // Try strict array parse first
        match serde_json::from_str::<Vec<A2uiMessage>>(json) {
            Ok(messages) => return Ok(self.process_messages(messages)),
            Err(e) => {
                eprintln!("[A2UI processor] Strict array parse failed: {}", e);
            }
        }

        // Fallback: parse as array of generic Values, then try each individually
        if let Ok(values) = serde_json::from_str::<Vec<serde_json::Value>>(json) {
            let mut events = Vec::new();
            for (i, val) in values.iter().enumerate() {
                match serde_json::from_value::<A2uiMessage>(val.clone()) {
                    Ok(msg) => events.extend(self.process_message(msg)),
                    Err(e) => {
                        eprintln!(
                            "[A2UI processor] Skipping message[{}]: {}",
                            i, e
                        );
                    }
                }
            }
            if !events.is_empty() {
                return Ok(events);
            }
        }

        // Last resort: try as single message
        let message: A2uiMessage = serde_json::from_str(json)?;
        Ok(self.process_message(message))
    }

    /// Attempt to repair malformed JSON from LLM output.
    ///
    /// Handles common LLM JSON issues:
    /// - JavaScript-style comments (`//` and `/* */`)
    /// - Trailing commas before `]` or `}`
    /// - Unclosed strings, brackets, and braces (token-limit truncation)
    /// - Incomplete trailing entries (key without value)
    /// - Truncated arrays/objects (removes last incomplete element)
    fn repair_json(json: &str) -> String {
        // If it already parses, return as-is
        if serde_json::from_str::<serde_json::Value>(json).is_ok() {
            return json.to_string();
        }

        eprintln!("[A2UI repair] JSON is invalid, attempting repair");

        // Step 1: Strip JS-style comments (// and /* */)
        let mut repaired = Self::strip_json_comments(json);

        // Step 2: Remove trailing commas before ] or }
        repaired = Self::fix_trailing_commas(&repaired);

        // Quick check after comment/comma fixes
        if serde_json::from_str::<serde_json::Value>(&repaired).is_ok() {
            eprintln!(
                "[A2UI repair] Fixed by stripping comments/trailing commas"
            );
            return repaired;
        }

        // Step 2b: Fix lines with unbalanced braces
        // GPT-4.1 often omits the outer closing brace on component lines,
        // e.g. `{"id": "x", "component": {"Column": {"children": ...}}},`
        //       should be `{"id": "x", "component": {"Column": {"children": ...}}}},`
        repaired = Self::fix_unbalanced_lines(&repaired);

        if serde_json::from_str::<serde_json::Value>(&repaired).is_ok() {
            eprintln!(
                "[A2UI repair] Fixed by balancing braces on lines"
            );
            return repaired;
        }

        // Step 3: Fix truncation — close unclosed brackets/braces/strings
        repaired = repaired.trim_end().to_string();

        // Remove trailing comma
        while repaired.ends_with(',') {
            repaired.pop();
        }

        let mut stack: Vec<char> = Vec::new();
        let mut in_string = false;
        let mut escape_next = false;

        for ch in repaired.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }
            if ch == '\\' && in_string {
                escape_next = true;
                continue;
            }
            if ch == '"' {
                in_string = !in_string;
                continue;
            }
            if in_string {
                continue;
            }
            match ch {
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ']' | '}' => { stack.pop(); }
                _ => {}
            }
        }

        if in_string {
            repaired.push('"');
        }

        let trimmed = repaired.trim_end();
        if trimmed.ends_with(':') || trimmed.ends_with(',') {
            repaired = trimmed
                .trim_end_matches(|c: char| c == ':' || c == ',')
                .to_string();
        }

        while let Some(closer) = stack.pop() {
            repaired.push(closer);
        }

        if serde_json::from_str::<serde_json::Value>(&repaired).is_ok() {
            eprintln!(
                "[A2UI repair] Fixed by closing brackets ({} -> {} bytes)",
                json.len(),
                repaired.len()
            );
            return repaired;
        }

        // Step 4: Try removing the last incomplete array element
        // Find the last complete element by searching backwards for "},\n"
        if let Some(fixed) = Self::truncate_to_last_complete_element(
            &repaired,
        ) {
            if serde_json::from_str::<serde_json::Value>(&fixed).is_ok() {
                eprintln!(
                    "[A2UI repair] Fixed by truncating ({} -> {} bytes)",
                    json.len(),
                    fixed.len()
                );
                return fixed;
            }
        }

        eprintln!("[A2UI repair] Repair failed, returning original");
        json.to_string()
    }

    /// Strip JavaScript-style comments from JSON text.
    /// Handles `// line comment` and `/* block comment */`.
    fn strip_json_comments(json: &str) -> String {
        let mut result = String::with_capacity(json.len());
        let chars: Vec<char> = json.chars().collect();
        let len = chars.len();
        let mut i = 0;
        let mut in_string = false;
        let mut escape_next = false;

        while i < len {
            if escape_next {
                escape_next = false;
                result.push(chars[i]);
                i += 1;
                continue;
            }
            if in_string {
                if chars[i] == '\\' {
                    escape_next = true;
                } else if chars[i] == '"' {
                    in_string = false;
                }
                result.push(chars[i]);
                i += 1;
                continue;
            }
            if chars[i] == '"' {
                in_string = true;
                result.push(chars[i]);
                i += 1;
                continue;
            }
            // Check for // line comment
            if i + 1 < len && chars[i] == '/' && chars[i + 1] == '/' {
                // Skip until end of line
                while i < len && chars[i] != '\n' {
                    i += 1;
                }
                continue;
            }
            // Check for /* block comment */
            if i + 1 < len && chars[i] == '/' && chars[i + 1] == '*' {
                i += 2;
                while i + 1 < len
                    && !(chars[i] == '*' && chars[i + 1] == '/')
                {
                    i += 1;
                }
                i += 2; // skip */
                continue;
            }
            result.push(chars[i]);
            i += 1;
        }
        result
    }

    /// Fix trailing commas before `]` or `}`.
    fn fix_trailing_commas(json: &str) -> String {
        let mut result = String::with_capacity(json.len());
        let chars: Vec<char> = json.chars().collect();
        let len = chars.len();
        let mut in_string = false;
        let mut escape_next = false;

        for i in 0..len {
            if escape_next {
                escape_next = false;
                result.push(chars[i]);
                continue;
            }
            if in_string {
                if chars[i] == '\\' {
                    escape_next = true;
                } else if chars[i] == '"' {
                    in_string = false;
                }
                result.push(chars[i]);
                continue;
            }
            if chars[i] == '"' {
                in_string = true;
                result.push(chars[i]);
                continue;
            }
            // Skip comma if followed only by whitespace and then ] or }
            if chars[i] == ',' {
                let rest = &chars[i + 1..];
                let next_non_ws = rest.iter().find(|c| !c.is_whitespace());
                if matches!(next_non_ws, Some(']') | Some('}')) {
                    continue; // skip this trailing comma
                }
            }
            result.push(chars[i]);
        }
        result
    }

    /// Fix lines where braces are unbalanced.
    ///
    /// GPT-4.1 often generates component definition lines with missing or
    /// extra closing braces. This handles both cases:
    /// - Missing `}`: when a new `{"id":` line starts while previous has
    ///   unclosed braces, insert missing `}` before it.
    /// - Extra `}`: when a line has more `}` than `{`, remove the excess
    ///   closing braces from the end.
    fn fix_unbalanced_lines(json: &str) -> String {
        let mut result = String::with_capacity(json.len() + 512);
        let mut running_balance: i32 = 0;

        for line in json.split('\n') {
            let trimmed = line.trim();

            // When a new component starts, check if previous was unclosed
            if trimmed.starts_with("{\"id\"") && running_balance > 0 {
                let result_trimmed = result.trim_end().to_string();
                result.clear();
                let stripped = result_trimmed.trim_end_matches(',');
                let had_comma =
                    result_trimmed.len() > stripped.len();
                result.push_str(stripped);
                for _ in 0..running_balance {
                    result.push('}');
                }
                if had_comma {
                    result.push(',');
                }
                result.push('\n');
                running_balance = 0;
            }

            // Count braces/brackets on this line, respecting strings
            let mut line_balance: i32 = 0;
            let mut in_str = false;
            let mut esc = false;
            for ch in trimmed.chars() {
                if esc { esc = false; continue; }
                if ch == '\\' && in_str { esc = true; continue; }
                if ch == '"' { in_str = !in_str; continue; }
                if in_str { continue; }
                match ch {
                    '{' | '[' => line_balance += 1,
                    '}' | ']' => line_balance -= 1,
                    _ => {}
                }
            }

            // Fix extra closing braces on component lines
            // Remove `}` at positions where running balance goes negative
            if line_balance < 0 && trimmed.contains("\"id\"") {
                let excess = (-line_balance) as usize;
                let mut fixed = String::with_capacity(trimmed.len());
                let mut removed = 0usize;
                let mut bal: i32 = 0;
                let mut in_s = false;
                let mut esc2 = false;
                for ch in trimmed.chars() {
                    if esc2 { esc2 = false; fixed.push(ch); continue; }
                    if ch == '\\' && in_s {
                        esc2 = true; fixed.push(ch); continue;
                    }
                    if ch == '"' { in_s = !in_s; fixed.push(ch); continue; }
                    if in_s { fixed.push(ch); continue; }
                    match ch {
                        '{' | '[' => { bal += 1; fixed.push(ch); }
                        '}' | ']' => {
                            bal -= 1;
                            if bal < 0 && removed < excess {
                                // Skip this excess closer
                                bal += 1;
                                removed += 1;
                            } else {
                                fixed.push(ch);
                            }
                        }
                        _ => { fixed.push(ch); }
                    }
                }
                let indent = line.len() - line.trim_start().len();
                result.push_str(&line[..indent]);
                result.push_str(&fixed);
                result.push('\n');
                running_balance += line_balance + removed as i32;
            } else {
                running_balance += line_balance;
                result.push_str(line);
                result.push('\n');
            }
        }

        // Remove trailing newline added by split
        if result.ends_with('\n') && !json.ends_with('\n') {
            result.pop();
        }
        result
    }

    /// Try to truncate JSON to the last complete top-level array element.
    fn truncate_to_last_complete_element(json: &str) -> Option<String> {
        // Find positions of "}, " or "},\n" at nesting depth 1
        // (top-level array elements in A2UI are objects)
        let chars: Vec<char> = json.chars().collect();
        let len = chars.len();
        let mut depth = 0;
        let mut in_string = false;
        let mut escape_next = false;
        let mut last_complete_end = None;

        for i in 0..len {
            if escape_next {
                escape_next = false;
                continue;
            }
            if in_string {
                if chars[i] == '\\' {
                    escape_next = true;
                } else if chars[i] == '"' {
                    in_string = false;
                }
                continue;
            }
            if chars[i] == '"' {
                in_string = true;
                continue;
            }
            match chars[i] {
                '[' | '{' => depth += 1,
                ']' | '}' => {
                    depth -= 1;
                    // depth==1 means we just closed a top-level array
                    // element (the outer [ is depth 0 after open)
                    if depth == 1 && chars[i] == '}' {
                        last_complete_end = Some(i);
                    }
                }
                _ => {}
            }
        }

        let end = last_complete_end?;
        // Build: everything up to and including this }, then close ]
        let mut fixed: String = chars[..=end].iter().collect();
        // Remove trailing comma if any
        let trimmed = fixed.trim_end();
        if trimmed.ends_with(',') {
            fixed = trimmed
                .trim_end_matches(',')
                .to_string();
        }
        fixed.push_str("\n]");
        Some(fixed)
    }

    /// Take pending user actions (clears the queue)
    pub fn take_pending_actions(&mut self) -> Vec<UserAction> {
        std::mem::take(&mut self.pending_actions)
    }

    /// Queue a user action to be sent
    pub fn queue_user_action(&mut self, action: UserAction) {
        self.pending_actions.push(action);
    }

    /// Create a user action from a button click
    ///
    /// The `scope` parameter is used for template rendering - it provides the base path
    /// for resolving relative paths in action context (e.g., "/products/0" for the first item)
    pub fn create_action(
        &self,
        surface_id: &str,
        component_id: &str,
        action_def: &ActionDefinition,
        scope: Option<&str>,
    ) -> UserAction {
        let mut context = HashMap::new();

        // Resolve context values from data model
        if let Some(data_model) = self.get_data_model(surface_id) {
            for item in &action_def.context {
                let value = match &item.value {
                    ActionValue::String(sv) => match sv {
                        StringValue::Literal { literal_string } => {
                            serde_json::Value::String(literal_string.clone())
                        }
                        StringValue::Path { path } => {
                            let resolved_path = resolve_path(path, scope);
                            data_model
                                .get(&resolved_path)
                                .cloned()
                                .unwrap_or(serde_json::Value::Null)
                        }
                    },
                    ActionValue::Number(nv) => match nv {
                        NumberValue::Literal { literal_number } => {
                            serde_json::json!(*literal_number)
                        }
                        NumberValue::Path { path } => {
                            let resolved_path = resolve_path(path, scope);
                            data_model
                                .get(&resolved_path)
                                .cloned()
                                .unwrap_or(serde_json::Value::Null)
                        }
                    },
                    ActionValue::Boolean(bv) => match bv {
                        BooleanValue::Literal { literal_boolean } => {
                            serde_json::Value::Bool(*literal_boolean)
                        }
                        BooleanValue::Path { path } => {
                            let resolved_path = resolve_path(path, scope);
                            data_model
                                .get(&resolved_path)
                                .cloned()
                                .unwrap_or(serde_json::Value::Null)
                        }
                    },
                };
                context.insert(item.key.clone(), value);
            }
        }

        UserAction {
            surface_id: surface_id.to_string(),
            action: UserActionPayload {
                name: action_def.name.clone(),
                context,
            },
            component_id: Some(component_id.to_string()),
        }
    }

    // ========================================================================
    // Private processing methods
    // ========================================================================

    fn process_begin_rendering(&mut self, msg: BeginRendering) -> Vec<ProcessorEvent> {
        let surface = Surface::new(msg.surface_id.clone(), msg.root, msg.styles);

        // Create data model for this surface
        self.data_models.get_or_create(&msg.surface_id);

        // Store surface
        self.surfaces.insert(msg.surface_id.clone(), surface);

        vec![ProcessorEvent::SurfaceCreated(SurfaceCreatedEvent {
            surface_id: msg.surface_id,
        })]
    }

    fn process_surface_update(&mut self, msg: SurfaceUpdate) -> Vec<ProcessorEvent> {
        let surface = match self.surfaces.get_mut(&msg.surface_id) {
            Some(s) => s,
            None => {
                // Create surface implicitly if it doesn't exist
                let surface = Surface::new(msg.surface_id.clone(), String::new(), None);
                self.surfaces.insert(msg.surface_id.clone(), surface);
                self.data_models.get_or_create(&msg.surface_id);
                self.surfaces.get_mut(&msg.surface_id).unwrap()
            }
        };

        let mut updated_ids = Vec::new();

        for component in msg.components {
            updated_ids.push(component.id.clone());
            surface.components.insert(component.id.clone(), component);
        }

        surface.mark_dirty();

        vec![ProcessorEvent::SurfaceUpdated(SurfaceUpdatedEvent {
            surface_id: msg.surface_id,
            updated_components: updated_ids,
        })]
    }

    fn process_data_model_update(&mut self, msg: DataModelUpdate) -> Vec<ProcessorEvent> {
        let data_model = self.data_models.get_or_create(&msg.surface_id);

        let mut updated_paths = Vec::new();

        for content in &msg.contents {
            let full_path = if msg.path == "/" {
                format!("/{}", content.key)
            } else {
                format!("{}/{}", msg.path.trim_end_matches('/'), content.key)
            };
            updated_paths.push(full_path);
        }

        data_model.apply_updates(&msg.path, &msg.contents);

        // Mark surface as needing redraw
        if let Some(surface) = self.surfaces.get_mut(&msg.surface_id) {
            surface.mark_dirty();
        }

        vec![ProcessorEvent::DataModelUpdated(DataModelUpdatedEvent {
            surface_id: msg.surface_id,
            updated_paths,
        })]
    }

    fn process_delete_surface(&mut self, msg: DeleteSurface) -> Vec<ProcessorEvent> {
        self.surfaces.remove(&msg.surface_id);
        self.data_models.remove(&msg.surface_id);

        vec![ProcessorEvent::SurfaceDeleted(SurfaceDeletedEvent {
            surface_id: msg.surface_id,
        })]
    }
}

/// Resolve a path with optional scope prefix.
/// - If path starts with `/`, it's absolute (use as-is)
/// - Otherwise, it's relative (prepend scope)
fn resolve_path(path: &str, scope: Option<&str>) -> String {
    if path.starts_with('/') {
        // Absolute path
        path.to_string()
    } else if let Some(scope_prefix) = scope {
        // Relative path with scope
        format!("{}/{}", scope_prefix, path)
    } else {
        // Relative path without scope - treat as absolute
        format!("/{}", path)
    }
}

/// Resolve a StringValue to an actual string using the data model
pub fn resolve_string_value(value: &StringValue, data_model: &DataModel) -> String {
    resolve_string_value_scoped(value, data_model, None)
}

/// Resolve a StringValue with optional scope for template rendering
pub fn resolve_string_value_scoped(
    value: &StringValue,
    data_model: &DataModel,
    scope: Option<&str>,
) -> String {
    match value {
        StringValue::Literal { literal_string } => literal_string.clone(),
        StringValue::Path { path } => {
            let resolved_path = resolve_path(path, scope);
            data_model
                .get_string(&resolved_path)
                .map(|s| s.to_string())
                .unwrap_or_default()
        }
    }
}

/// Resolve a NumberValue to an actual number using the data model
pub fn resolve_number_value(value: &NumberValue, data_model: &DataModel) -> f64 {
    resolve_number_value_scoped(value, data_model, None)
}

/// Resolve a NumberValue with optional scope for template rendering
pub fn resolve_number_value_scoped(
    value: &NumberValue,
    data_model: &DataModel,
    scope: Option<&str>,
) -> f64 {
    match value {
        NumberValue::Literal { literal_number } => *literal_number,
        NumberValue::Path { path } => {
            let resolved_path = resolve_path(path, scope);
            data_model.get_number(&resolved_path).unwrap_or(0.0)
        }
    }
}

/// Resolve a BooleanValue to an actual boolean using the data model
pub fn resolve_boolean_value(value: &BooleanValue, data_model: &DataModel) -> bool {
    resolve_boolean_value_scoped(value, data_model, None)
}

/// Resolve a BooleanValue with optional scope for template rendering
pub fn resolve_boolean_value_scoped(
    value: &BooleanValue,
    data_model: &DataModel,
    scope: Option<&str>,
) -> bool {
    match value {
        BooleanValue::Literal { literal_boolean } => *literal_boolean,
        BooleanValue::Path { path } => {
            let resolved_path = resolve_path(path, scope);
            data_model.get_bool(&resolved_path).unwrap_or(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_begin_rendering() {
        let mut processor = A2uiMessageProcessor::with_standard_catalog();

        let msg = A2uiMessage::BeginRendering(BeginRendering {
            surface_id: "main".to_string(),
            root: "root".to_string(),
            styles: None,
        });

        let events = processor.process_message(msg);

        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            ProcessorEvent::SurfaceCreated(e) if e.surface_id == "main"
        ));

        assert!(processor.get_surface("main").is_some());
        assert!(processor.get_data_model("main").is_some());
    }

    #[test]
    fn test_process_surface_update() {
        let mut processor = A2uiMessageProcessor::with_standard_catalog();

        // First create the surface
        processor.process_message(A2uiMessage::BeginRendering(BeginRendering {
            surface_id: "main".to_string(),
            root: "root".to_string(),
            styles: None,
        }));

        // Then update it
        let msg = A2uiMessage::SurfaceUpdate(SurfaceUpdate {
            surface_id: "main".to_string(),
            components: vec![ComponentDefinition {
                id: "title".to_string(),
                weight: None,
                component: ComponentType::Text(TextComponent {
                    text: StringValue::literal("Hello"),
                    usage_hint: Some(TextUsageHint::H1),
                }),
            }],
        });

        let events = processor.process_message(msg);

        assert_eq!(events.len(), 1);
        assert!(matches!(
            &events[0],
            ProcessorEvent::SurfaceUpdated(e) if e.surface_id == "main"
        ));

        let surface = processor.get_surface("main").unwrap();
        assert!(surface.get_component("title").is_some());
    }

    #[test]
    fn test_process_data_model_update() {
        let mut processor = A2uiMessageProcessor::with_standard_catalog();

        // Create surface
        processor.process_message(A2uiMessage::BeginRendering(BeginRendering {
            surface_id: "main".to_string(),
            root: "root".to_string(),
            styles: None,
        }));

        // Update data model
        let msg = A2uiMessage::DataModelUpdate(DataModelUpdate {
            surface_id: "main".to_string(),
            path: "/".to_string(),
            contents: vec![DataContent {
                key: "name".to_string(),
                value: DataValue::ValueString("Alice".to_string()),
            }],
        });

        let events = processor.process_message(msg);

        assert_eq!(events.len(), 1);

        let data_model = processor.get_data_model("main").unwrap();
        assert_eq!(data_model.get_string("/name"), Some("Alice"));
    }

    #[test]
    fn test_resolve_string_value() {
        let mut data_model = DataModel::new();
        data_model.set_string("/user/name", "Bob");

        // Test literal
        let literal = StringValue::literal("Hello");
        assert_eq!(resolve_string_value(&literal, &data_model), "Hello");

        // Test path
        let path = StringValue::path("/user/name");
        assert_eq!(resolve_string_value(&path, &data_model), "Bob");
    }
}
