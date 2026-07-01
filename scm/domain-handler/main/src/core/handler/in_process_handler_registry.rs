//! `HandlerRegistry` impl for [`InProcessHandlerRegistry`] — RwLock-backed in-process store.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use crate::api::DeregisterHandlerRequest;
use crate::api::DeregisterHandlerResponse;
use crate::api::EmptinessRequest;
use crate::api::EmptinessResponse;
use crate::api::GetHandlerRequest;
use crate::api::GetHandlerResponse;
use crate::api::Handler;
use crate::api::HandlerError;
use crate::api::HandlerRegistry;
use crate::api::IdRequest;
use crate::api::InProcessHandlerRegistry;
use crate::api::LenRequest;
use crate::api::LenResponse;
use crate::api::ListIdsRequest;
use crate::api::ListIdsResponse;
use crate::api::RegisterHandlerRequest;
use crate::api::RegisterHandlerResponse;

impl<Req, Resp> RegisterHandlerRequest<Req, Resp>
where
    Req: Send + 'static,
    Resp: Send + 'static,
{
    /// Create a new registration request for the given handler.
    pub fn new(handler: Arc<dyn Handler<Request = Req, Response = Resp>>) -> Self {
        Self { handler }
    }
}

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

    fn register(
        &self,
        req: RegisterHandlerRequest<Req, Resp>,
    ) -> Result<RegisterHandlerResponse, HandlerError> {
        let id = req.handler.id(IdRequest)?.id;
        self.handlers.write().insert(id, req.handler);
        Ok(RegisterHandlerResponse)
    }

    fn deregister(
        &self,
        req: DeregisterHandlerRequest,
    ) -> Result<DeregisterHandlerResponse, HandlerError> {
        let was_present = self.handlers.write().remove(&req.id).is_some();
        Ok(DeregisterHandlerResponse { was_present })
    }

    fn get(
        &self,
        req: GetHandlerRequest,
    ) -> Result<GetHandlerResponse<Req, Resp>, HandlerError> {
        let handler = self.handlers.read().get(&req.id).cloned();
        Ok(GetHandlerResponse { handler })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, HandlerError> {
        let guard = self.handlers.read();
        let mut ids: Vec<String> = guard.keys().cloned().collect();
        ids.sort();
        Ok(ListIdsResponse { ids })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, HandlerError> {
        Ok(LenResponse {
            count: self.handlers.read().len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ExecuteRequest;
    use crate::api::HandlerContext;
    use crate::api::IdResponse;
    use async_trait::async_trait;

    struct InProcessHandlerRegistryFixture;

    #[async_trait]
    impl Handler for InProcessHandlerRegistryFixture {
        type Request = String;
        type Response = String;

        fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
            Ok(IdResponse {
                id: "fixture".to_string(),
            })
        }
        async fn execute(
            &self,
            req: ExecuteRequest<'_, String>,
        ) -> Result<String, HandlerError> {
            Ok(req.req)
        }
    }

    fn make_registry() -> InProcessHandlerRegistry<String, String> {
        InProcessHandlerRegistry::new()
    }

    #[test]
    fn test_register_handler_is_retrievable_happy() {
        let reg = make_registry();
        reg.register(RegisterHandlerRequest::new(Arc::new(
            InProcessHandlerRegistryFixture,
        )))
        .unwrap();
        let handler = reg
            .get(GetHandlerRequest {
                id: "fixture".to_string(),
            })
            .unwrap()
            .handler;
        assert!(handler.is_some());
        assert_eq!(handler.unwrap().id(IdRequest).unwrap().id, "fixture");
    }

    #[test]
    fn test_deregister_existing_handler_returns_true_happy() {
        let reg = make_registry();
        reg.register(RegisterHandlerRequest::new(Arc::new(
            InProcessHandlerRegistryFixture,
        )))
        .unwrap();
        let req = DeregisterHandlerRequest {
            id: "fixture".to_string(),
        };
        assert!(reg.deregister(req).unwrap().was_present);
    }

    #[test]
    fn test_deregister_missing_id_returns_false_error() {
        let reg = make_registry();
        let req = DeregisterHandlerRequest {
            id: "nonexistent".to_string(),
        };
        assert!(!reg.deregister(req).unwrap().was_present);
    }

    #[test]
    fn test_get_missing_id_returns_none_error() {
        let reg = make_registry();
        let req = GetHandlerRequest {
            id: "missing".to_string(),
        };
        assert!(reg.get(req).unwrap().handler.is_none());
    }

    #[test]
    fn test_list_ids_returns_sorted_ids_edge() {
        let reg = make_registry();
        reg.register(RegisterHandlerRequest::new(Arc::new(
            InProcessHandlerRegistryFixture,
        )))
        .unwrap();
        let ids = reg.list_ids(ListIdsRequest).unwrap().ids;
        assert_eq!(ids, vec!["fixture"]);
    }

    #[test]
    fn test_len_empty_registry_returns_zero_edge() {
        let reg = make_registry();
        assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    }

    #[test]
    fn test_is_empty_new_registry_returns_true_edge() {
        let reg = make_registry();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }

    #[test]
    fn test_register_replaces_existing_handler_with_same_id_edge() {
        let reg = make_registry();
        reg.register(RegisterHandlerRequest::new(Arc::new(
            InProcessHandlerRegistryFixture,
        )))
        .unwrap();
        reg.register(RegisterHandlerRequest::new(Arc::new(
            InProcessHandlerRegistryFixture,
        )))
        .unwrap();
        assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    }

    #[test]
    fn test_len_returns_registered_count_happy() {
        let reg = make_registry();
        assert_eq!(reg.len(LenRequest).unwrap().count, 0);
        reg.register(RegisterHandlerRequest::new(Arc::new(
            InProcessHandlerRegistryFixture,
        )))
        .unwrap();
        assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    }
}
