//! `ServiceRegistry` trait — local decoupling boundary for looking up domain services by name.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{
    ListNamesRequest, ListNamesResponse, ServiceLookupRequest, ServiceLookupResponse,
};

/// A thread-safe registry of domain [`Service`](super::Service) instances, looked up by name.
///
/// Declared locally so `api/` never references `edge_application_service::ServiceRegistry` directly
/// in a type position (SEA `no_foreign_type`). Any real `ServiceRegistry` implementor satisfies
/// this automatically via the blanket impl in `core/`.
pub trait ServiceRegistry: Send + Sync {
    /// The request type for services stored in this registry.
    type Request: Send + 'static;

    /// The response type for services stored in this registry.
    type Response: Send + 'static;

    /// Return the names of all registered services.
    fn list_names(&self, req: ListNamesRequest) -> Result<ListNamesResponse, HandlerError>;

    /// Look up a service by name.
    fn get(
        &self,
        req: ServiceLookupRequest,
    ) -> Result<ServiceLookupResponse<Self::Request, Self::Response>, HandlerError>;
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use async_trait::async_trait;

    use super::*;
    use crate::api::handler::traits::service::Service;

    struct NoopService;

    #[async_trait]
    impl Service for NoopService {
        type Request = ();
        type Response = ();

        async fn execute(&self, _req: ()) -> Result<(), HandlerError> {
            Ok(())
        }
    }

    struct FixedRegistry {
        names: Vec<String>,
    }

    impl ServiceRegistry for FixedRegistry {
        type Request = ();
        type Response = ();

        fn list_names(&self, _req: ListNamesRequest) -> Result<ListNamesResponse, HandlerError> {
            Ok(ListNamesResponse {
                names: self.names.clone(),
            })
        }

        fn get(
            &self,
            req: ServiceLookupRequest,
        ) -> Result<ServiceLookupResponse<(), ()>, HandlerError> {
            if self.names.contains(&req.name) {
                Ok(ServiceLookupResponse {
                    service: Some(Arc::new(NoopService)),
                })
            } else {
                Ok(ServiceLookupResponse { service: None })
            }
        }
    }

    #[test]
    fn test_list_names_returns_registered_names_happy() {
        let registry = FixedRegistry {
            names: vec!["svc-1".to_string()],
        };
        assert_eq!(
            registry.list_names(ListNamesRequest).unwrap().names,
            vec!["svc-1"]
        );
    }

    #[test]
    fn test_get_unknown_name_returns_none_error() {
        let registry = FixedRegistry { names: vec![] };
        let resp = registry
            .get(ServiceLookupRequest {
                name: "missing".to_string(),
            })
            .unwrap();
        assert!(resp.service.is_none());
    }

    #[test]
    fn test_get_empty_registry_list_names_returns_empty_edge() {
        let registry = FixedRegistry { names: vec![] };
        assert!(registry
            .list_names(ListNamesRequest)
            .unwrap()
            .names
            .is_empty());
    }
}
