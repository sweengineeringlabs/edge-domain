//! [`HandlerRegistry`] trait — contract for handler instance registries.

use std::sync::Arc;

use crate::api::handler::Handler;

/// Thread-safe registry of [`Handler`] instances keyed by id.
pub trait HandlerRegistry<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    /// Register a handler, replacing any existing entry with the same id.
    fn register(&self, handler: Arc<dyn Handler<Request, Response>>);

    /// Deregister the handler with the given id. Returns `true` if removed.
    fn deregister(&self, id: &str) -> bool;

    /// Look up a handler by id. Returns `None` if not registered.
    fn get(&self, id: &str) -> Option<Arc<dyn Handler<Request, Response>>>;

    /// Snapshot of registered handler ids. Order is unspecified.
    fn list_ids(&self) -> Vec<String>;

    /// Number of currently registered handlers.
    fn len(&self) -> usize;

    /// Whether the registry has no handlers.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub use crate::api::handler::types::HandlerRegistry as HandlerRegistryImpl;
