//! `OutboundRegistry` — thread-safe registry of outbound handles keyed by name.

use std::collections::HashMap;

use parking_lot::RwLock;

/// Thread-safe registry of outbound handles keyed by name.
///
/// ## Example
///
/// ```rust
/// use edge_domain::OutboundRegistry;
///
/// let reg: OutboundRegistry<String> = OutboundRegistry::new();
/// reg.register("anthropic", "https://api.anthropic.com".to_string());
/// assert_eq!(reg.get("anthropic").as_deref(), Some("https://api.anthropic.com"));
/// ```
pub struct OutboundRegistry<H: Clone + Send + Sync> {
    pub(crate) handles: RwLock<HashMap<String, H>>,
}

impl<H: Clone + Send + Sync> OutboundRegistry<H> {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            handles: RwLock::new(HashMap::new()),
        }
    }

    /// Register a handle under `name`, replacing any existing entry.
    pub fn register(&self, name: impl Into<String>, handle: H) {
        self.handles.write().insert(name.into(), handle);
    }

    /// Deregister the handle with the given name. Returns `true` if removed.
    pub fn deregister(&self, name: &str) -> bool {
        self.handles.write().remove(name).is_some()
    }

    /// Look up a handle by name. Returns a clone on hit, `None` on miss.
    pub fn get(&self, name: &str) -> Option<H> {
        self.handles.read().get(name).cloned()
    }

    /// Snapshot of registered names. Order is unspecified.
    pub fn names(&self) -> Vec<String> {
        self.handles.read().keys().cloned().collect()
    }

    /// Number of currently registered handles.
    pub fn len(&self) -> usize {
        self.handles.read().len()
    }

    /// Whether the registry holds no handles.
    pub fn is_empty(&self) -> bool {
        self.handles.read().is_empty()
    }
}

impl<H: Clone + Send + Sync> Default for OutboundRegistry<H> {
    fn default() -> Self {
        Self::new()
    }
}
