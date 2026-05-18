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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: register
    #[test]
    fn test_register_stores_handle_retrievable_by_name() {
        let reg: OutboundRegistry<u32> = OutboundRegistry::new();
        reg.register("svc", 42u32);
        assert_eq!(reg.get("svc"), Some(42));
    }

    /// @covers: get
    #[test]
    fn test_get_returns_none_for_unregistered_name() {
        let reg: OutboundRegistry<String> = OutboundRegistry::new();
        assert!(reg.get("missing").is_none());
    }

    /// @covers: deregister
    #[test]
    fn test_deregister_removes_handle_and_returns_true() {
        let reg: OutboundRegistry<String> = OutboundRegistry::new();
        reg.register("svc", "url".to_string());
        assert!(reg.deregister("svc"));
        assert!(reg.get("svc").is_none());
    }

    /// @covers: names
    #[test]
    fn test_names_returns_all_registered_keys() {
        let reg: OutboundRegistry<u8> = OutboundRegistry::new();
        reg.register("a", 1u8);
        reg.register("b", 2u8);
        let mut names = reg.names();
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
    }

    /// @covers: len
    #[test]
    fn test_len_returns_correct_count() {
        let reg: OutboundRegistry<i32> = OutboundRegistry::new();
        assert_eq!(reg.len(), 0);
        reg.register("x", 0i32);
        assert_eq!(reg.len(), 1);
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_reflects_registry_state() {
        let reg: OutboundRegistry<i32> = OutboundRegistry::new();
        assert!(reg.is_empty());
        reg.register("x", 0i32);
        assert!(!reg.is_empty());
    }
}
