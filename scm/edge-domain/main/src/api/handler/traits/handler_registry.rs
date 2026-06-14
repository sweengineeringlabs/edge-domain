//! [`HandlerRegistry`] trait — contract for handler instance registries.

use std::sync::Arc;

use crate::api::handler::Handler;

/// Thread-safe registry of [`Handler`] instances keyed by id.
///
/// The in-process reference implementation is
/// [`crate::api::handler::types::in_process_handler_registry::InProcessHandlerRegistry`].
pub trait HandlerRegistry: Send + Sync {
    /// The request type accepted by handlers in this registry.
    type Request: Send + 'static;
    /// The response type produced by handlers in this registry.
    type Response: Send + 'static;

    /// Register a handler, replacing any existing entry with the same id.
    fn register(&self, handler: Arc<dyn Handler<Request = Self::Request, Response = Self::Response>>);

    /// Deregister the handler with the given id. Returns `true` if removed.
    fn deregister(&self, id: &str) -> bool;

    /// Look up a handler by id. Returns `None` if not registered.
    fn get(&self, id: &str) -> Option<Arc<dyn Handler<Request = Self::Request, Response = Self::Response>>>;

    /// Snapshot of registered handler ids. Order is unspecified.
    fn list_ids(&self) -> Vec<String>;

    /// Number of currently registered handlers.
    fn len(&self) -> usize;

    /// Whether the registry has no handlers.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
