//! Integration tests for `EchoHandler` and the `echo_handler` factory.

use std::sync::Arc;
use edge_domain::{echo_handler, EchoHandler, Handler};

/// @covers: echo_handler
#[test]
fn test_echo_handler_factory_returns_arc_handler() {
    let _: Arc<dyn Handler<String, String>> = echo_handler("id", "/path");
}

/// @covers: echo_handler
#[tokio::test]
async fn test_echo_handler_returns_request_as_response() {
    let h = echo_handler::<String>("echo", "/echo");
    let result = h.execute("hello".to_string()).await.unwrap();
    assert_eq!(result, "hello");
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_id_matches_constructor_arg() {
    let h: Arc<dyn Handler<String, String>> = echo_handler("my-handler", "/api/v1");
    assert_eq!(h.id(), "my-handler");
}

/// @covers: echo_handler
#[test]
fn test_echo_handler_pattern_matches_constructor_arg() {
    let h: Arc<dyn Handler<String, String>> = echo_handler("id", "/api/v1/things");
    assert_eq!(h.pattern(), "/api/v1/things");
}

/// @covers: EchoHandler
#[tokio::test]
async fn test_echo_handler_struct_health_check_defaults_to_true() {
    let h = EchoHandler::<String>::new("id", "/p");
    assert!(h.health_check().await);
}

/// @covers: EchoHandler
#[tokio::test]
async fn test_echo_handler_works_with_numeric_type() {
    let h: Arc<dyn Handler<u64, u64>> = echo_handler("num", "/num");
    assert_eq!(h.execute(42u64).await.unwrap(), 42u64);
}
