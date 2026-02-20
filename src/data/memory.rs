use moly_kit::aitk::utils::asynchronous::spawn;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::shared::utils::filesystem;

const MEMORY_DIR: &str = "memory";
const MEMORY_FILENAME: &str = "memories.json";

/// A single remembered fact about the user.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Memory {
    pub id: String,
    pub content: String,
    pub created_at: String,
}

/// Persistent store for user memories (inner data).
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct MemoryStoreInner {
    memories: Vec<Memory>,
}

/// Thread-safe handle to the persistent memory store.
///
/// Cloning this handle shares the same underlying data.
#[derive(Clone, Debug, Default)]
pub struct MemoryStore(Arc<Mutex<MemoryStoreInner>>);

impl MemoryStore {
    /// Loads the memory store from disk, returning a default if none exists.
    pub async fn load() -> Self {
        let fs = filesystem::global();
        let inner = match fs.read_json::<MemoryStoreInner>(&memory_path()).await
        {
            Ok(store) => store,
            Err(_) => {
                log::info!(
                    "No memories file found, starting with empty store."
                );
                MemoryStoreInner::default()
            }
        };
        Self(Arc::new(Mutex::new(inner)))
    }

    /// Returns a snapshot of all memories.
    pub fn memories(&self) -> Vec<Memory> {
        self.0.lock().unwrap().memories.clone()
    }

    /// Returns `true` if there are no saved memories.
    pub fn is_empty(&self) -> bool {
        self.0.lock().unwrap().memories.is_empty()
    }

    /// Adds a new memory and persists.
    pub fn add(&self, content: String) -> Memory {
        let mut inner = self.0.lock().unwrap();
        let memory = Memory {
            id: Uuid::new_v4().to_string(),
            content,
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        inner.memories.push(memory.clone());
        Self::persist(&inner);
        memory
    }

    /// Removes a memory by id. Returns `true` if found and removed.
    pub fn remove(&self, id: &str) -> bool {
        let mut inner = self.0.lock().unwrap();
        let len_before = inner.memories.len();
        inner.memories.retain(|m| m.id != id);
        let removed = inner.memories.len() < len_before;
        if removed {
            Self::persist(&inner);
        }
        removed
    }

    /// Removes all memories and persists.
    pub fn clear(&self) {
        let mut inner = self.0.lock().unwrap();
        inner.memories.clear();
        Self::persist(&inner);
    }

    /// Renders all memories as a bullet list for system prompt injection.
    pub fn format_for_prompt(&self) -> String {
        let inner = self.0.lock().unwrap();
        if inner.memories.is_empty() {
            return String::new();
        }

        inner
            .memories
            .iter()
            .map(|m| format!("- {}", m.content))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn persist(inner: &MemoryStoreInner) {
        let clone = inner.clone();
        spawn(async move {
            match filesystem::global()
                .queue_write_json(memory_path(), &clone)
                .await
            {
                Ok(()) => (),
                Err(e) => log::error!("Failed to write memories: {:?}", e),
            }
        });
    }
}

fn memory_path() -> PathBuf {
    Path::new(MEMORY_DIR).join(MEMORY_FILENAME)
}
