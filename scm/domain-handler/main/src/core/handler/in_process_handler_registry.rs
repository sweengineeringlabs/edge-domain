//! `HandlerRegistry` impl for [`InProcessHandlerRegistry`] — RwLock-backed in-process store.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::Handler;
use crate::api::HandlerRegistry;
use crate::api::InProcessHandlerRegistry;

impl<Req, Resp> InProcessHandlerRegistry<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    pub(crate) fn new() -> Self {
        Self { handlers: RwLock::new(HashMap::new()) }
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

impl<Req: Send + 'static, Resp: Send + 'static> HandlerRegistry
    for InProcessHandlerRegistry<Req, Resp>
{
    type Request = Req;
    type Response = Resp;

    fn register(&self, handler: Arc<dyn Handler<Request = Req, Response = Resp>>) {
        self.handlers
            .write()
            .insert(handler.id().to_string(), handler);
    }

    fn deregister(&self, id: &str) -> bool {
        self.handlers.write().remove(id).is_some()
    }

    fn get(&self, id: &str) -> Option<Arc<dyn Handler<Request = Req, Response = Resp>>> {
        self.handlers.read().get(id).cloned()
    }

    fn list_ids(&self) -> Vec<String> {
        let guard = self.handlers.read();
        let mut ids: Vec<String> = guard.keys().cloned().collect();
        ids.sort();
        ids
    }

    fn len(&self) -> usize {
        self.handlers.read().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::HandlerContext;
    use crate::api::HandlerError;
    use async_trait::async_trait;

    struct InProcessHandlerRegistryFixture;

    #[async_trait]
    impl Handler for InProcessHandlerRegistryFixture {
        type Request = String;
        type Response = String;

        fn id(&self) -> &str {
            "fixture"
        }
        async fn execute(
            &self,
            req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
            Ok(req)
        }
    }

    fn make_registry() -> InProcessHandlerRegistry<String, String> {
        InProcessHandlerRegistry::new()
    }

    #[test]
    fn test_register_handler_is_retrievable_happy() {
        let reg = make_registry();
        reg.register(Arc::new(InProcessHandlerRegistryFixture));
        let handler = reg.get("fixture");
        assert!(handler.is_some());
        assert_eq!(handler.unwrap().id(), "fixture");
    }

    #[test]
    fn test_deregister_existing_handler_returns_true_happy() {
        let reg = make_registry();
        reg.register(Arc::new(InProcessHandlerRegistryFixture));
        assert!(reg.deregister("fixture"));
    }

    #[test]
    fn test_deregister_missing_id_returns_false_error() {
        let reg = make_registry();
        assert!(!reg.deregister("nonexistent"));
    }

    #[test]
    fn test_get_missing_id_returns_none_error() {
        let reg = make_registry();
        assert!(reg.get("missing").is_none());
    }

    #[test]
    fn test_list_ids_returns_sorted_ids_edge() {
        let reg = make_registry();
        reg.register(Arc::new(InProcessHandlerRegistryFixture));
        let ids = reg.list_ids();
        assert_eq!(ids, vec!["fixture"]);
    }

    #[test]
    fn test_len_empty_registry_returns_zero_edge() {
        let reg = make_registry();
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn test_is_empty_new_registry_returns_true_edge() {
        let reg = make_registry();
        assert!(reg.is_empty());
    }

    #[test]
    fn test_register_replaces_existing_handler_with_same_id_edge() {
        let reg = make_registry();
        reg.register(Arc::new(InProcessHandlerRegistryFixture));
        reg.register(Arc::new(InProcessHandlerRegistryFixture));
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_len_returns_registered_count_happy() {
        let reg = make_registry();
        assert_eq!(reg.len(), 0);
        reg.register(Arc::new(InProcessHandlerRegistryFixture));
        assert_eq!(reg.len(), 1);
    }
}
