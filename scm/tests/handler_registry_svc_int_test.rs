#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — HandlerRegistry is exported from the crate root.

use edge_domain::Domain;
use edge_domain::EchoHandler;
use edge_domain::HandlerRegistry;
use std::sync::Arc;

#[test]
fn test_handler_registry_svc_facade_register_and_retrieve() {
    let reg = Domain::new_handler_registry::<String, String>();
    reg.register(Arc::new(EchoHandler::<String>::new("echo", "*")));
    assert!(reg.get("echo").is_some());
}

#[test]
fn test_handler_registry_svc_facade_missing_id_returns_none() {
    let reg = Domain::new_handler_registry::<String, String>();
    assert!(reg.get("absent").is_none());
}
