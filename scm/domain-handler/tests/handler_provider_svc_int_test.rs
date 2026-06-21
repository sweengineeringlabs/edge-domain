//! Integration tests — `HandlerProvider` trait.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{
    Handler, HandlerContext, HandlerError, HandlerProvider, HandlerRegistry,
};
use edge_domain_observe::StdObserveFactory;
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

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
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &bus, observer.as_ref());
    assert_eq!(block_on(h.execute("ping".into(), ctx)).unwrap(), "ping");
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
    struct Ping;
    #[async_trait]
    impl Handler for Ping {
        type Request = String;
        type Response = String;

        fn id(&self) -> &str {
            "ping"
        }
        async fn execute(
            &self,
            _req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
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
    struct Tmp;
    #[async_trait]
    impl Handler for Tmp {
        type Request = String;
        type Response = String;

        fn id(&self) -> &str {
            "tmp"
        }
        async fn execute(
            &self,
            _req: String,
            _ctx: HandlerContext<'_>,
        ) -> Result<String, HandlerError> {
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
    let h = Prov::echo_handler("", "");
    assert_eq!(h.id(), "");
}

/// @covers: HandlerProvider::in_process_registry — infallible (no error path; empty state is valid)
#[test]
fn test_in_process_registry_empty_state_is_not_an_error_error() {
    let reg = Prov::in_process_registry::<String, String>();
    assert_eq!(reg.len(), 0);
}

/// @covers: HandlerProvider::noop_handler_factory — constructs a NoopHandlerFactory
#[test]
fn test_noop_handler_factory_constructs_instance_happy() {
    use edge_domain_handler::NoopHandlerFactory;
    let _f: NoopHandlerFactory = Prov::noop_handler_factory();
}

/// @covers: HandlerProvider::noop_handler_factory — infallible (no error path; documents absence)
#[test]
fn test_noop_handler_factory_is_always_infallible_error() {
    use edge_domain_handler::NoopHandlerFactory;
    let _f: NoopHandlerFactory = Prov::noop_handler_factory();
}

/// @covers: HandlerProvider::noop_handler_factory — Copy semantics allow multiple uses
#[test]
fn test_noop_handler_factory_copy_allows_multiple_uses_edge() {
    use edge_domain_handler::{HandlerBootstrap, NoopHandlerFactory};
    let f: NoopHandlerFactory = Prov::noop_handler_factory();
    let g = f; // Copy
    let _r1 = NoopHandlerFactory::build(()).unwrap();
    let _ = (f, g);
}
