//! `HandlerRegistry` trait — stores and retrieves [`Handler`] instances by id.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::traits::Handler;
use crate::api::handler::types::{
    DeregisterHandlerRequest, DeregisterHandlerResponse, EmptinessRequest, EmptinessResponse,
    GetHandlerRequest, GetHandlerResponse, LenRequest, LenResponse, ListIdsRequest,
    ListIdsResponse, RegisterHandlerRequest, RegisterHandlerResponse,
};

/// A thread-safe registry that stores and retrieves [`Handler`] instances by id.
pub trait HandlerRegistry: Send + Sync {
    /// The request type for handlers stored in this registry.
    type Request: Send + 'static;

    /// The response type for handlers stored in this registry.
    type Response: Send + 'static;

    /// Register a handler, replacing any existing entry with the same id.
    fn register(
        &self,
        req: RegisterHandlerRequest<Self::Request, Self::Response>,
    ) -> Result<RegisterHandlerResponse, HandlerError>;

    /// Remove the handler with the given id. Returns `true` if it existed.
    fn deregister(
        &self,
        req: DeregisterHandlerRequest,
    ) -> Result<DeregisterHandlerResponse, HandlerError>;

    /// Look up a handler by id.
    fn get(
        &self,
        req: GetHandlerRequest,
    ) -> Result<GetHandlerResponse<Self::Request, Self::Response>, HandlerError>;

    /// Return all registered handler ids.
    fn list_ids(&self, req: ListIdsRequest) -> Result<ListIdsResponse, HandlerError>;

    /// Return the number of registered handlers.
    fn len(&self, req: LenRequest) -> Result<LenResponse, HandlerError>;

    /// Return `true` if no handlers are registered.
    fn is_empty(&self, _req: EmptinessRequest) -> Result<EmptinessResponse, HandlerError> {
        Ok(EmptinessResponse {
            empty: self.len(LenRequest)?.count == 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::handler::types::{ExecuteRequest, IdRequest, IdResponse};
    use async_trait::async_trait;
    use std::sync::Arc;

    struct Noop;

    #[async_trait]
    impl Handler for Noop {
        type Request = String;
        type Response = String;

        fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
            Ok(IdResponse {
                id: "noop".to_string(),
            })
        }
        async fn execute(
            &self,
            req: ExecuteRequest<'_, String>,
        ) -> Result<String, HandlerError> {
            Ok(req.req)
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

        fn register(
            &self,
            req: RegisterHandlerRequest<String, String>,
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
        ) -> Result<GetHandlerResponse<String, String>, HandlerError> {
            let handler = self.handlers.read().get(&req.id).cloned();
            Ok(GetHandlerResponse { handler })
        }
        fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, HandlerError> {
            Ok(ListIdsResponse {
                ids: self.handlers.read().keys().cloned().collect(),
            })
        }
        fn len(&self, _req: LenRequest) -> Result<LenResponse, HandlerError> {
            Ok(LenResponse {
                count: self.handlers.read().len(),
            })
        }
    }

    #[test]
    fn test_is_empty_no_handlers_returns_true_happy() {
        let reg = HandlerRegistryFixture::new();
        assert!(reg.is_empty(EmptinessRequest).unwrap().empty);
    }

    #[test]
    fn test_register_handler_increases_len_happy() {
        let reg = HandlerRegistryFixture::new();
        reg.register(RegisterHandlerRequest::new(Arc::new(Noop))).unwrap();
        assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    }

    #[test]
    fn test_deregister_missing_id_returns_false_error() {
        let reg = HandlerRegistryFixture::new();
        let req = DeregisterHandlerRequest {
            id: "nonexistent".to_string(),
        };
        assert!(!reg.deregister(req).unwrap().was_present);
    }
}
