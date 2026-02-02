//! A2UI Data Model
//!
//! The DataModel is a reactive data store using JSON Pointer paths for access.
//! Components subscribe to paths and are automatically notified when data changes.

use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// A reactive data model that stores values accessible via JSON Pointer paths.
///
/// The data model supports:
/// - Getting and setting values by path
/// - Subscribing to path changes
/// - Nested object and array access
///
/// # Path Format
///
/// Paths follow JSON Pointer (RFC 6901) format:
/// - `/` - root
/// - `/foo` - property "foo"
/// - `/foo/bar` - nested property
/// - `/items/0` - array element at index 0
/// - `/items/0/name` - property of array element
///
/// # Example
///
/// ```rust,ignore
/// let mut model = DataModel::new();
///
/// // Set values
/// model.set("/user/name", json!("Alice"));
/// model.set("/items", json!([{"id": 1}, {"id": 2}]));
///
/// // Get values
/// let name = model.get_string("/user/name"); // Some("Alice")
/// let id = model.get_number("/items/0/id");  // Some(1.0)
/// ```
#[derive(Debug, Clone)]
pub struct DataModel {
    /// The root data value
    data: Value,

    /// Set of paths that have been modified since last clear
    dirty_paths: HashSet<String>,

    /// Version counter for change detection
    version: u64,
}

impl Default for DataModel {
    fn default() -> Self {
        Self::new()
    }
}

impl DataModel {
    /// Create a new empty data model
    pub fn new() -> Self {
        DataModel {
            data: Value::Object(serde_json::Map::new()),
            dirty_paths: HashSet::new(),
            version: 0,
        }
    }

    /// Create a data model with initial data
    pub fn with_data(data: Value) -> Self {
        DataModel {
            data,
            dirty_paths: HashSet::new(),
            version: 0,
        }
    }

    /// Get the current version number
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Check if a path has been modified
    pub fn is_dirty(&self, path: &str) -> bool {
        // Check if the exact path is dirty
        if self.dirty_paths.contains(path) {
            return true;
        }

        // Check if any parent path is dirty (means children might have changed)
        for dirty_path in &self.dirty_paths {
            if path.starts_with(dirty_path) || dirty_path.starts_with(path) {
                return true;
            }
        }

        false
    }

    /// Clear the dirty flags
    pub fn clear_dirty(&mut self) {
        self.dirty_paths.clear();
    }

    /// Get all dirty paths
    pub fn dirty_paths(&self) -> &HashSet<String> {
        &self.dirty_paths
    }

    /// Get a value at the given path
    pub fn get(&self, path: &str) -> Option<&Value> {
        self.get_by_pointer(path)
    }

    /// Get a string value at the given path
    pub fn get_string(&self, path: &str) -> Option<&str> {
        self.get(path).and_then(|v| v.as_str())
    }

    /// Get a number value at the given path
    pub fn get_number(&self, path: &str) -> Option<f64> {
        self.get(path).and_then(|v| v.as_f64())
    }

    /// Get a boolean value at the given path
    pub fn get_bool(&self, path: &str) -> Option<bool> {
        self.get(path).and_then(|v| v.as_bool())
    }

    /// Get an array value at the given path
    pub fn get_array(&self, path: &str) -> Option<&Vec<Value>> {
        self.get(path).and_then(|v| v.as_array())
    }

    /// Get an object value at the given path
    pub fn get_object(&self, path: &str) -> Option<&serde_json::Map<String, Value>> {
        self.get(path).and_then(|v| v.as_object())
    }

    /// Set a value at the given path
    ///
    /// Creates intermediate objects/arrays as needed.
    pub fn set(&mut self, path: &str, value: Value) {
        if self.set_by_pointer(path, value) {
            self.dirty_paths.insert(path.to_string());
            self.version += 1;
        }
    }

    /// Set a string value at the given path
    pub fn set_string(&mut self, path: &str, value: impl Into<String>) {
        self.set(path, Value::String(value.into()));
    }

    /// Set a number value at the given path
    pub fn set_number(&mut self, path: &str, value: f64) {
        self.set(path, serde_json::json!(value));
    }

    /// Set a boolean value at the given path
    pub fn set_bool(&mut self, path: &str, value: bool) {
        self.set(path, Value::Bool(value));
    }

    /// Delete a value at the given path
    pub fn delete(&mut self, path: &str) -> bool {
        if self.delete_by_pointer(path) {
            self.dirty_paths.insert(path.to_string());
            self.version += 1;
            true
        } else {
            false
        }
    }

    /// Merge updates from a DataModelUpdate message
    pub fn apply_updates(&mut self, base_path: &str, contents: &[super::message::DataContent]) {
        for content in contents {
            let full_path = if base_path == "/" {
                format!("/{}", content.key)
            } else {
                format!("{}/{}", base_path.trim_end_matches('/'), content.key)
            };

            let value = self.data_value_to_json(&content.value);
            self.set(&full_path, value);
        }
    }

    /// Convert DataValue to serde_json::Value
    fn data_value_to_json(&self, dv: &super::message::DataValue) -> Value {
        match dv {
            super::message::DataValue::ValueString(s) => Value::String(s.clone()),
            super::message::DataValue::ValueNumber(n) => serde_json::json!(n),
            super::message::DataValue::ValueBoolean(b) => Value::Bool(*b),
            super::message::DataValue::ValueMap(contents) => {
                let mut map = serde_json::Map::new();
                for c in contents {
                    map.insert(c.key.clone(), self.data_value_to_json(&c.value));
                }
                Value::Object(map)
            }
            super::message::DataValue::ValueArray(items) => {
                Value::Array(items.iter().map(|v| self.data_value_to_json(v)).collect())
            }
        }
    }

    /// Get the entire data as a Value
    pub fn as_value(&self) -> &Value {
        &self.data
    }

    /// Replace the entire data model
    pub fn replace(&mut self, data: Value) {
        self.data = data;
        self.dirty_paths.insert("/".to_string());
        self.version += 1;
    }

    // ========================================================================
    // Private helpers
    // ========================================================================

    /// Parse a JSON Pointer path into segments
    fn parse_pointer(path: &str) -> Vec<&str> {
        if path.is_empty() || path == "/" {
            return vec![];
        }

        path.trim_start_matches('/')
            .split('/')
            .map(|s| {
                // Unescape JSON Pointer special sequences
                // ~1 -> /
                // ~0 -> ~
                s // TODO: implement proper unescaping if needed
            })
            .collect()
    }

    /// Get a value by JSON Pointer path
    fn get_by_pointer(&self, path: &str) -> Option<&Value> {
        let segments = Self::parse_pointer(path);

        let mut current = &self.data;
        for segment in segments {
            current = match current {
                Value::Object(map) => map.get(segment)?,
                Value::Array(arr) => {
                    let index: usize = segment.parse().ok()?;
                    arr.get(index)?
                }
                _ => return None,
            };
        }

        Some(current)
    }

    /// Set a value by JSON Pointer path, creating intermediate structures
    fn set_by_pointer(&mut self, path: &str, value: Value) -> bool {
        let segments = Self::parse_pointer(path);

        if segments.is_empty() {
            self.data = value;
            return true;
        }

        // Navigate to parent, creating objects as needed
        let mut current = &mut self.data;

        for (i, segment) in segments.iter().enumerate() {
            let is_last = i == segments.len() - 1;

            if is_last {
                // Set the final value
                match current {
                    Value::Object(map) => {
                        map.insert(segment.to_string(), value);
                        return true;
                    }
                    Value::Array(arr) => {
                        if let Ok(index) = segment.parse::<usize>() {
                            if index < arr.len() {
                                arr[index] = value;
                                return true;
                            } else if index == arr.len() {
                                arr.push(value);
                                return true;
                            }
                        }
                        return false;
                    }
                    _ => return false,
                }
            } else {
                // Navigate or create intermediate structure
                let next_segment = segments.get(i + 1).unwrap();
                let next_is_array_index = next_segment.parse::<usize>().is_ok();

                match current {
                    Value::Object(map) => {
                        if !map.contains_key(*segment) {
                            let new_value = if next_is_array_index {
                                Value::Array(vec![])
                            } else {
                                Value::Object(serde_json::Map::new())
                            };
                            map.insert(segment.to_string(), new_value);
                        }
                        current = map.get_mut(*segment).unwrap();
                    }
                    Value::Array(arr) => {
                        if let Ok(index) = segment.parse::<usize>() {
                            if index >= arr.len() {
                                // Extend array with nulls if needed
                                while arr.len() <= index {
                                    arr.push(Value::Null);
                                }
                            }
                            current = &mut arr[index];
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        // Replace with object
                        *current = Value::Object(serde_json::Map::new());
                        if let Value::Object(map) = current {
                            let new_value = if next_is_array_index {
                                Value::Array(vec![])
                            } else {
                                Value::Object(serde_json::Map::new())
                            };
                            map.insert(segment.to_string(), new_value);
                            current = map.get_mut(*segment).unwrap();
                        }
                    }
                }
            }
        }

        false
    }

    /// Delete a value by JSON Pointer path
    fn delete_by_pointer(&mut self, path: &str) -> bool {
        let segments = Self::parse_pointer(path);

        if segments.is_empty() {
            self.data = Value::Object(serde_json::Map::new());
            return true;
        }

        // Navigate to parent
        let parent_segments = &segments[..segments.len() - 1];
        let last_segment = segments.last().unwrap();

        let mut current = &mut self.data;
        for segment in parent_segments {
            current = match current {
                Value::Object(map) => match map.get_mut(*segment) {
                    Some(v) => v,
                    None => return false,
                },
                Value::Array(arr) => {
                    if let Ok(index) = segment.parse::<usize>() {
                        match arr.get_mut(index) {
                            Some(v) => v,
                            None => return false,
                        }
                    } else {
                        return false;
                    }
                }
                _ => return false,
            };
        }

        // Delete from parent
        match current {
            Value::Object(map) => map.remove(*last_segment).is_some(),
            Value::Array(arr) => {
                if let Ok(index) = last_segment.parse::<usize>() {
                    if index < arr.len() {
                        arr.remove(index);
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

/// A collection of surfaces with their data models
#[derive(Debug, Default)]
pub struct SurfaceDataModels {
    models: HashMap<String, DataModel>,
}

impl SurfaceDataModels {
    /// Create a new empty collection
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
        }
    }

    /// Get or create a data model for a surface
    pub fn get_or_create(&mut self, surface_id: &str) -> &mut DataModel {
        self.models
            .entry(surface_id.to_string())
            .or_insert_with(DataModel::new)
    }

    /// Get a data model for a surface
    pub fn get(&self, surface_id: &str) -> Option<&DataModel> {
        self.models.get(surface_id)
    }

    /// Get a mutable data model for a surface
    pub fn get_mut(&mut self, surface_id: &str) -> Option<&mut DataModel> {
        self.models.get_mut(surface_id)
    }

    /// Remove a surface's data model
    pub fn remove(&mut self, surface_id: &str) -> Option<DataModel> {
        self.models.remove(surface_id)
    }

    /// Check if a surface exists
    pub fn contains(&self, surface_id: &str) -> bool {
        self.models.contains_key(surface_id)
    }

    /// Get all surface IDs
    pub fn surface_ids(&self) -> impl Iterator<Item = &String> {
        self.models.keys()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_set_basic() {
        let mut model = DataModel::new();

        model.set("/name", json!("Alice"));
        assert_eq!(model.get_string("/name"), Some("Alice"));

        model.set("/count", json!(42));
        assert_eq!(model.get_number("/count"), Some(42.0));

        model.set("/enabled", json!(true));
        assert_eq!(model.get_bool("/enabled"), Some(true));
    }

    #[test]
    fn test_nested_paths() {
        let mut model = DataModel::new();

        model.set("/user/name", json!("Alice"));
        model.set("/user/email", json!("alice@example.com"));

        assert_eq!(model.get_string("/user/name"), Some("Alice"));
        assert_eq!(model.get_string("/user/email"), Some("alice@example.com"));
    }

    #[test]
    fn test_array_access() {
        let mut model = DataModel::new();

        model.set("/items", json!([{"id": 1}, {"id": 2}, {"id": 3}]));

        assert_eq!(model.get_number("/items/0/id"), Some(1.0));
        assert_eq!(model.get_number("/items/1/id"), Some(2.0));
        assert_eq!(model.get_number("/items/2/id"), Some(3.0));
    }

    #[test]
    fn test_dirty_tracking() {
        let mut model = DataModel::new();

        assert!(!model.is_dirty("/name"));

        model.set("/name", json!("Alice"));
        assert!(model.is_dirty("/name"));

        model.clear_dirty();
        assert!(!model.is_dirty("/name"));
    }

    #[test]
    fn test_delete() {
        let mut model = DataModel::new();

        model.set("/name", json!("Alice"));
        assert!(model.get("/name").is_some());

        model.delete("/name");
        assert!(model.get("/name").is_none());
    }

    #[test]
    fn test_version() {
        let mut model = DataModel::new();

        let v0 = model.version();
        model.set("/name", json!("Alice"));
        let v1 = model.version();

        assert!(v1 > v0);
    }
}
