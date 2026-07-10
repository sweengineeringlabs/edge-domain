#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — HandlerRegistry is exported from the crate root.

use edge_domain::Domain;
use edge_domain::EchoHandler;
use edge_domain::HandlerRegistry;
use edge_domain_handler::{HandlerLookupRequest, RegisterHandlerRequest};
use std::sync::Arc;

#[test]
fn test_handler_registry_svc_facade_register_and_retrieve() {
    let reg = Domain.new_handler_registry::<String, String>();
    let handler: Arc<dyn edge_domain::Handler<Request = String, Response = String>> =
        Arc::new(EchoHandler::from(("echo", "*")));
    reg.register(RegisterHandlerRequest::new(handler)).unwrap();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "echo".to_string()
        })
        .unwrap()
        .handler
        .is_some());
}

#[test]
fn test_handler_registry_svc_facade_missing_id_returns_none() {
    let reg = Domain.new_handler_registry::<String, String>();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "absent".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}
