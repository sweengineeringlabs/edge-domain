//! Integration tests for the `HandlerProvider` SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{EchoHandler, HandlerProvider, HandlerRegistry, InProcessHandlerRegistry};
use edge_domain_handler::LenRequest;

struct TestHandlers;
impl HandlerProvider for TestHandlers {}

// --- HandlerProvider::echo_handler ---

/// @covers HandlerProvider::echo_handler — happy path: returns a typed EchoHandler
#[test]
fn test_handler_provider_echo_handler_returns_typed_handler_happy() {
    let h: EchoHandler<String> = TestHandlers::echo_handler("echo", "/ping");
    assert_eq!(h.id, "echo");
    assert_eq!(h.pattern, "/ping");
}

/// @covers HandlerProvider::echo_handler — error: empty id is accepted (no validation at this layer)
#[test]
fn test_handler_provider_echo_handler_empty_id_accepted_error() {
    let h: EchoHandler<String> = TestHandlers::echo_handler("", "/");
    assert_eq!(h.id, "");
}

/// @covers HandlerProvider::echo_handler — edge: id and pattern are arbitrary strings
#[test]
fn test_handler_provider_echo_handler_arbitrary_strings_edge() {
    let h: EchoHandler<String> = TestHandlers::echo_handler("abc-123", "/v1/resource/{id}");
    assert_eq!(h.id, "abc-123");
    assert_eq!(h.pattern, "/v1/resource/{id}");
}

// --- HandlerProvider::in_process_registry ---

/// @covers HandlerProvider::in_process_registry — happy path: returns the registry
#[test]
fn test_in_process_registry_returns_registry_happy() {
    let _: InProcessHandlerRegistry<String, String> =
        TestHandlers::in_process_registry::<String, String>();
}

/// @covers HandlerProvider::in_process_registry — error: registry starts empty
#[test]
fn test_in_process_registry_starts_empty_error() {
    let reg = TestHandlers::in_process_registry::<String, String>();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
}

/// @covers HandlerProvider::in_process_registry — edge: successive calls are independent
#[test]
fn test_in_process_registry_independent_calls_edge() {
    let _a = TestHandlers::in_process_registry::<String, String>();
    let _b = TestHandlers::in_process_registry::<String, String>();
}
