//! In-process [`HandlerRegistry`] implementation backed by `parking_lot::RwLock`.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::handler::Handler;
use crate::api::handler::HandlerRegistry;

/// Thread-safe, in-process handler registry backed by a `RwLock<HashMap>`.
pub(crate) struct InProcessHandlerRegistry<Req, Resp> {
    handlers: RwLock<HashMap<String, Arc<dyn Handler<Request = Req, Response = Resp>>>>,
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

// impl HandlerRegistry for InProcessHandlerRegistry
impl<Req, Resp> HandlerRegistry for InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    type Request = Req;
    type Response = Resp;

    fn register(&self, handler: Arc<dyn Handler<Request = Req, Response = Resp>>) {
        self.handlers
            .write()
            .insert(handler.id().to_owned(), handler);
    }

    fn deregister(&self, id: &str) -> bool {
        self.handlers.write().remove(id).is_some()
    }

    fn get(&self, id: &str) -> Option<Arc<dyn Handler<Request = Req, Response = Resp>>> {
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
