//! Integration tests — `EchoHandler` type.

use edge_domain_handler::{EchoHandler, Handler};
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

/// @covers: EchoHandler::execute — returns request unchanged
#[test]
fn test_execute_returns_request_unchanged_happy() {
    let h = EchoHandler::<String>::new("echo", "/");
    let result = block_on(h.execute("hello".into()));
    assert_eq!(result.unwrap(), "hello");
}

/// @covers: EchoHandler::id — returns configured id
#[test]
fn test_id_returns_configured_id_happy() {
    let h = EchoHandler::<String>::new("my-echo", "/*");
    assert_eq!(h.id(), "my-echo");
}

/// @covers: EchoHandler::pattern — returns configured pattern
#[test]
fn test_pattern_returns_configured_pattern_happy() {
    let h = EchoHandler::<String>::new("e", "/path");
    assert_eq!(h.pattern(), "/path");
}

/// @covers: EchoHandler::execute — empty string returns empty string
#[test]
fn test_execute_empty_string_returns_empty_string_edge() {
    let h = EchoHandler::<String>::new("e", "/");
    assert_eq!(block_on(h.execute("".into())).unwrap(), "");
}

/// @covers: EchoHandler::health_check default
#[test]
fn test_health_check_returns_true_happy() {
    let h = EchoHandler::<String>::new("e", "/");
    assert!(block_on(h.health_check()));
}

/// @covers: EchoHandler::execute_with_context delegates to execute
#[test]
fn test_execute_with_context_returns_same_value_happy() {
    let h = EchoHandler::<String>::new("e", "/");
    let ctx = SecurityContext::unauthenticated();
    let result = block_on(h.execute_with_context("world".into(), ctx));
    assert_eq!(result.unwrap(), "world");
}

/// @covers: EchoHandler — usable as dyn Handler
#[test]
fn test_echo_handler_usable_as_dyn_handler_edge() {
    use std::sync::Arc;
    let h: Arc<dyn Handler<String, String>> = Arc::new(EchoHandler::new("dyn", "/"));
    let result = block_on(h.execute("dyn-test".into()));
    assert_eq!(result.unwrap(), "dyn-test");
}
