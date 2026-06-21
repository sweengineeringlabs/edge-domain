//! `HandlerRegistry` trait — stores and retrieves [`Handler`] instances by id.

use std::sync::Arc;

use crate::api::handler::traits::Handler;

/// A thread-safe registry that stores and retrieves [`Handler`] instances by id.
pub trait HandlerRegistry: Send + Sync {
    /// The request type for handlers stored in this registry.
    type Request: Send + 'static;

    /// The response type for handlers stored in this registry.
    type Response: Send + 'static;

    /// Register a handler, replacing any existing entry with the same id.
    fn register(
        &self,
        handler: Arc<dyn Handler<Request = Self::Request, Response = Self::Response>>,
    );

    /// Remove the handler with the given id. Returns `true` if it existed.
    fn deregister(&self, id: &str) -> bool;

    /// Look up a handler by id.
    fn get(
        &self,
        id: &str,
    ) -> Option<Arc<dyn Handler<Request = Self::Request, Response = Self::Response>>>;

    /// Return all registered handler ids.
    fn list_ids(&self) -> Vec<String>;

    /// Return the number of registered handlers.
    fn len(&self) -> usize;

    /// Return `true` if no handlers are registered.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::handler::errors::HandlerError;
    use async_trait::async_trait;

    struct Noop;

    #[async_trait]
    impl Handler for Noop {
        type Request = String;
        type Response = String;

        fn id(&self) -> &str {
            "noop"
        }
        async fn execute(
            &self,
            req: String,
            _ctx: crate::api::handler::types::HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
            Ok(req)
        }
    }

    struct HandlerRegistryFixture {
        handlers: parking_lot::RwLock<
            std::collections::HashMap<
                String,
                Arc<dyn Handler<Request = String, Response = String>>,
            >,
        >,
    }

    impl HandlerRegistryFixture {
        fn new() -> Self {
            Self {
                handlers: parking_lot::RwLock::new(std::collections::HashMap::new()),
            }
        }
    }

    impl HandlerRegistry for HandlerRegistryFixture {
        type Request = String;
        type Response = String;

        fn register(&self, handler: Arc<dyn Handler<Request = String, Response = String>>) {
            self.handlers
                .write()
                .insert(handler.id().to_string(), handler);
        }
        fn deregister(&self, id: &str) -> bool {
            self.handlers.write().remove(id).is_some()
        }
        fn get(&self, id: &str) -> Option<Arc<dyn Handler<Request = String, Response = String>>> {
            self.handlers.read().get(id).cloned()
        }
        fn list_ids(&self) -> Vec<String> {
            self.handlers.read().keys().cloned().collect()
        }
        fn len(&self) -> usize {
            self.handlers.read().len()
        }
    }

    #[test]
    fn test_is_empty_no_handlers_returns_true_happy() {
        let reg = HandlerRegistryFixture::new();
        assert!(reg.is_empty());
    }

    #[test]
    fn test_register_handler_increases_len_happy() {
        let reg = HandlerRegistryFixture::new();
        reg.register(Arc::new(Noop));
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_deregister_missing_id_returns_false_error() {
        let reg = HandlerRegistryFixture::new();
        assert!(!reg.deregister("nonexistent"));
    }
}
