//! `HandlerRegistry` — thread-safe registry of [`Handler`] implementations keyed by id.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::handler::Handler;

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
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
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
    fn default() -> Self {
        Self::new()
    }
}


