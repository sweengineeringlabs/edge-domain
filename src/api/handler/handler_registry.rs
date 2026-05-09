//! `HandlerRegistry` — thread-safe registry of [`Handler`] implementations keyed by id.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::Handler;

/// Registry of [`Handler`] instances keyed by [`Handler::id`].
///
/// Concurrency: guarded by a `parking_lot::RwLock` — lookups proceed in
/// parallel while registration and deregistration are serialized.
pub struct HandlerRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    pub(crate) handlers: RwLock<HashMap<String, Arc<dyn Handler<Request, Response>>>>,
}

impl<Request, Response> HandlerRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self { handlers: RwLock::new(HashMap::new()) }
    }

    /// Register a handler, replacing any existing entry with the same id.
    pub fn register(&self, handler: Arc<dyn Handler<Request, Response>>) {
        let id = handler.id().to_string();
        self.handlers.write().insert(id, handler);
    }

    /// Deregister the handler with the given id. Returns `true` if removed.
    pub fn deregister(&self, id: &str) -> bool {
        self.handlers.write().remove(id).is_some()
    }

    /// Look up a handler by id. Returns `None` if not registered.
    pub fn get(&self, id: &str) -> Option<Arc<dyn Handler<Request, Response>>> {
        self.handlers.read().get(id).cloned()
    }

    /// Snapshot of registered handler ids. Order is unspecified.
    pub fn list_ids(&self) -> Vec<String> {
        self.handlers.read().keys().cloned().collect()
    }

    /// Number of currently registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.read().len()
    }

    /// Whether the registry has no handlers.
    pub fn is_empty(&self) -> bool {
        self.handlers.read().is_empty()
    }
}

impl<Request, Response> Default for HandlerRegistry<Request, Response>
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::handler_error::HandlerError;
    use async_trait::async_trait;
    
    struct HandlerStub { id: String }
    #[async_trait]
    impl Handler<String, String> for HandlerStub {
        fn id(&self) -> &str { &self.id }
        fn pattern(&self) -> &str { "stub" }
        async fn execute(&self, req: String) -> Result<String, HandlerError> { Ok(req) }
    }
    fn stub(id: &str) -> Arc<dyn Handler<String, String>> {
        Arc::new(HandlerStub { id: id.to_string() })
    }

    /// @covers: register
    #[test]
    fn test_register_stores_handler_retrievable_by_id() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        reg.register(stub("a"));
        assert!(reg.get("a").is_some());
    }

    /// @covers: get
    #[test]
    fn test_get_returns_none_for_unregistered_id() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        assert!(reg.get("missing").is_none());
    }

    /// @covers: deregister
    #[test]
    fn test_deregister_removes_handler_and_returns_true() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        reg.register(stub("a"));
        assert!(reg.deregister("a"));
        assert!(reg.get("a").is_none());
    }

    /// @covers: list_ids
    #[test]
    fn test_list_ids_returns_all_registered_ids() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        reg.register(stub("a"));
        reg.register(stub("b"));
        let mut ids = reg.list_ids();
        ids.sort();
        assert_eq!(ids, vec!["a", "b"]);
    }

    /// @covers: len
    #[test]
    fn test_len_returns_correct_count() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        assert_eq!(reg.len(), 0);
        reg.register(stub("a"));
        assert_eq!(reg.len(), 1);
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_reflects_registry_state() {
        let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
        assert!(reg.is_empty());
        reg.register(stub("a"));
        assert!(!reg.is_empty());
    }
}
