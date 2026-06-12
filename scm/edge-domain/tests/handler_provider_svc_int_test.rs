//! Integration tests for the `HandlerProvider` SAF facade.

use edge_domain::{EchoHandler, Handler, HandlerProvider, InProcessHandlerRegistry};

struct TestHandlers;
impl HandlerProvider for TestHandlers {}

// --- HandlerProvider::echo_handler ---

/// @covers HandlerProvider::echo_handler — happy path: returns a typed EchoHandler
#[test]
fn test_handler_provider_echo_handler_returns_typed_handler_happy() {
    let h: EchoHandler<String> = TestHandlers::echo_handler("echo", "/ping");
    assert_eq!(h.id(), "echo");
    assert_eq!(h.pattern(), "/ping");
}

/// @covers HandlerProvider::echo_handler — error: empty id is accepted (no validation at this layer)
#[test]
fn test_handler_provider_echo_handler_empty_id_accepted_error() {
    let h: EchoHandler<String> = TestHandlers::echo_handler("", "/");
    assert_eq!(h.id(), "");
}

/// @covers HandlerProvider::echo_handler — edge: id and pattern are arbitrary strings
#[test]
fn test_handler_provider_echo_handler_arbitrary_strings_edge() {
    let h: EchoHandler<String> = TestHandlers::echo_handler("abc-123", "/v1/resource/{id}");
    assert_eq!(h.id(), "abc-123");
    assert_eq!(h.pattern(), "/v1/resource/{id}");
}

// --- HandlerProvider::in_process_registry ---

/// @covers HandlerProvider::in_process_registry — happy path: returns the registry marker
#[test]
fn test_in_process_registry_returns_marker_happy() {
    let _: InProcessHandlerRegistry = TestHandlers::in_process_registry();
}

/// @covers HandlerProvider::in_process_registry — error: marker is zero-size
#[test]
fn test_in_process_registry_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<InProcessHandlerRegistry>(), 0);
}

/// @covers HandlerProvider::in_process_registry — edge: successive calls are independent
#[test]
fn test_in_process_registry_independent_calls_edge() {
    let _a = TestHandlers::in_process_registry();
    let _b = TestHandlers::in_process_registry();
}

// --- HandlerProvider::request_context_builder ---

/// @covers HandlerProvider::request_context_builder — happy path: returns a fresh builder
#[test]
fn test_request_context_builder_returns_fresh_builder_happy() {
    let ctx = TestHandlers::request_context_builder().build();
    assert!(!ctx.authenticated);
}

/// @covers HandlerProvider::request_context_builder — error: builder with no subject yields None
#[test]
fn test_request_context_builder_no_subject_yields_none_error() {
    let ctx = TestHandlers::request_context_builder().build();
    assert!(ctx.subject.is_none());
}

/// @covers HandlerProvider::request_context_builder — edge: builder accepts all fields
#[test]
fn test_request_context_builder_full_fields_edge() {
    let ctx = TestHandlers::request_context_builder()
        .with_subject("alice")
        .with_trace_id("trace-42")
        .authenticated()
        .build();
    assert_eq!(ctx.subject.as_deref(), Some("alice"));
    assert_eq!(ctx.trace_id, "trace-42");
    assert!(ctx.authenticated);
}
