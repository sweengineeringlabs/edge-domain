//! [`InProcessHandlerRegistry`] — in-process, thread-safe handler registry.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::handler::traits::Handler;

/// An in-process, thread-safe handler registry backed by a `RwLock<HashMap>`.
///
/// The `HandlerRegistry` trait implementation lives in `core::handler::in_process_handler_registry`.
pub struct InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// The backing store.
    pub(crate) handlers: RwLock<HashMap<String, Arc<dyn Handler<Req, Resp>>>>,
}

impl<Req, Resp> InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }

    /// Return the number of registered handlers (direct accessor, does not require the trait).
    pub fn handler_count(&self) -> usize {
        self.handlers.read().len()
    }
}

impl<Req, Resp> Default for InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
