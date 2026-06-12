//! Integration tests — `HandlerProvider` trait.

use edge_domain_handler::{Handler, HandlerProvider, HandlerRegistry};
use futures::executor::block_on;
use std::sync::Arc;

struct Prov;
impl HandlerProvider for Prov {}

/// @covers: HandlerProvider::echo_handler — id and pattern set correctly
#[test]
fn test_echo_handler_id_and_pattern_set_correctly_happy() {
    let h = Prov::echo_handler("my-id", "/route");
    assert_eq!(h.id(), "my-id");
    assert_eq!(h.pattern(), "/route");
}

/// @covers: HandlerProvider::echo_handler — echoes request
#[test]
fn test_echo_handler_reflects_request_happy() {
    let h = Prov::echo_handler("e", "/");
    let result = block_on(h.execute("ping".into()));
    assert_eq!(result.unwrap(), "ping");
}

/// @covers: HandlerProvider::echo_handler — empty id/pattern edge
#[test]
fn test_echo_handler_empty_id_and_pattern_edge() {
    let h = Prov::echo_handler("", "");
    assert_eq!(h.id(), "");
    assert_eq!(h.pattern(), "");
}

/// @covers: HandlerProvider::in_process_registry — creates empty registry
#[test]
fn test_in_process_registry_creates_empty_registry_happy() {
    let reg = Prov::in_process_registry::<String, String>();
    assert!(reg.is_empty());
}

/// @covers: HandlerProvider::in_process_registry — registry is usable
#[test]
fn test_in_process_registry_register_and_retrieve_happy() {
    use async_trait::async_trait;
    use edge_domain_handler::HandlerError;

    struct Ping;
    #[async_trait]
    impl Handler<String, String> for Ping {
        fn id(&self) -> &str {
            "ping"
        }
        async fn execute(&self, _req: String) -> Result<String, HandlerError> {
            Ok("pong".into())
        }
    }

    let reg = Prov::in_process_registry::<String, String>();
    reg.register(Arc::new(Ping));
    assert_eq!(reg.len(), 1);
}

/// @covers: HandlerProvider::in_process_registry — empty after deregister
#[test]
fn test_in_process_registry_empty_after_deregister_edge() {
    use async_trait::async_trait;
    use edge_domain_handler::HandlerError;

    struct Tmp;
    #[async_trait]
    impl Handler<String, String> for Tmp {
        fn id(&self) -> &str {
            "tmp"
        }
        async fn execute(&self, _req: String) -> Result<String, HandlerError> {
            Ok(String::new())
        }
    }

    let reg = Prov::in_process_registry::<String, String>();
    reg.register(Arc::new(Tmp));
    reg.deregister("tmp");
    assert!(reg.is_empty());
}

/// @covers: HandlerProvider::echo_handler — infallible (no error path; demonstrates it never panics)
#[test]
fn test_echo_handler_always_constructs_without_error_error() {
    // echo_handler is infallible — any id/pattern produces a handler without panic.
    let h = Prov::echo_handler("", "");
    assert_eq!(h.id(), "");
}

/// @covers: HandlerProvider::in_process_registry — infallible (no error path; empty state is valid)
#[test]
fn test_in_process_registry_empty_state_is_not_an_error_error() {
    // in_process_registry is infallible — an empty registry is not an error condition.
    let reg = Prov::in_process_registry::<String, String>();
    assert_eq!(reg.len(), 0);
}
