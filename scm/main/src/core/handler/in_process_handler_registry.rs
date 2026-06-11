//! In-process [`HandlerRegistry`] implementation backed by `parking_lot::RwLock`.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::handler::Handler;
use crate::api::handler::HandlerRegistry;

/// Thread-safe, in-process handler registry backed by a `RwLock<HashMap>`.
pub(crate) struct InProcessHandlerRegistry<Req, Resp> {
    handlers: RwLock<HashMap<String, Arc<dyn Handler<Req, Resp>>>>,
}

impl<Req, Resp> InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    pub(crate) fn new() -> Self {
        Self {
            handlers: RwLock::new(HashMap::new()),
        }
    }
}

impl<Req, Resp> HandlerRegistry<Req, Resp> for InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    fn register(&self, handler: Arc<dyn Handler<Req, Resp>>) {
        self.handlers
            .write()
            .insert(handler.id().to_owned(), handler);
    }

    fn deregister(&self, id: &str) -> bool {
        self.handlers.write().remove(id).is_some()
    }

    fn get(&self, id: &str) -> Option<Arc<dyn Handler<Req, Resp>>> {
        self.handlers.read().get(id).cloned()
    }

    fn list_ids(&self) -> Vec<String> {
        self.handlers.read().keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.handlers.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_registry_is_empty() {
        let registry = InProcessHandlerRegistry::<String, String>::new();
        assert_eq!(registry.len(), 0);
    }
}
