#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — ServiceRegistry is exported from the crate root.
#![cfg(feature = "service")]

use edge_application::Domain;
use edge_application::Service;
use edge_application::ServiceError;
use edge_application_base::EmptyRequest;
use edge_application_service::{NameRequest, NameResponse, RegisterServiceRequest, ServiceLookupRequest};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

struct Greeter;
impl Service for Greeter {
    type Request = EmptyRequest;
    type Response = TextPayload;
    fn name(&self, _req: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse {
            name: "greeter".to_string(),
        })
    }
    fn execute(
        &self,
        _: EmptyRequest,
    ) -> futures::future::BoxFuture<'_, Result<TextPayload, ServiceError>> {
        Box::pin(async { Ok(TextPayload("hello".into())) })
    }
}

#[test]
fn test_service_registry_svc_facade_register_and_get() {
    let reg = Domain.new_service_registry::<EmptyRequest, TextPayload>();
    reg.register(&RegisterServiceRequest::new(Arc::new(Greeter)))
        .unwrap();
    assert!(reg
        .get(&ServiceLookupRequest {
            name: "greeter".to_string()
        })
        .unwrap()
        .service
        .is_some());
}

#[test]
fn test_service_registry_svc_facade_missing_name_returns_none() {
    let reg = Domain.new_service_registry::<EmptyRequest, TextPayload>();
    assert!(reg
        .get(&ServiceLookupRequest {
            name: "absent".to_string()
        })
        .unwrap()
        .service
        .is_none());
}
