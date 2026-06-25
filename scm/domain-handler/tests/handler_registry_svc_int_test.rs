//! Integration tests — `HandlerRegistry` trait via `InProcessHandlerRegistry`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{
    Handler, HandlerContext, HandlerError, HandlerRegistry, InProcessHandlerRegistry,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

struct Fixed {
    id: &'static str,
}

#[async_trait]
impl Handler for Fixed {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        self.id
    }
    async fn execute(&self, req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        Ok(req)
    }
}

fn make_reg() -> InProcessHandlerRegistry<String, String> {
    InProcessHandlerRegistry::default()
}

/// @covers: HandlerRegistry::register — handler is retrievable after registration
#[test]
fn test_register_handler_can_be_retrieved_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "alpha" }));
    let handler = reg.get("alpha");
    assert!(handler.is_some());
    assert_eq!(handler.unwrap().id(), "alpha");
}

/// @covers: HandlerRegistry::register — replaces duplicate id
#[test]
fn test_register_duplicate_id_replaces_existing_edge() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "alpha" }));
    reg.register(Arc::new(Fixed { id: "alpha" }));
    assert_eq!(reg.len(), 1);
}

/// @covers: HandlerRegistry::deregister — existing handler
#[test]
fn test_deregister_existing_handler_returns_true_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "beta" }));
    assert!(reg.deregister("beta"));
    assert!(reg.get("beta").is_none());
}

/// @covers: HandlerRegistry::deregister — missing id returns false
#[test]
fn test_deregister_missing_id_returns_false_error() {
    let reg = make_reg();
    assert!(!reg.deregister("ghost"));
}

/// @covers: HandlerRegistry::get — missing id
#[test]
fn test_get_missing_id_returns_none_error() {
    let reg = make_reg();
    assert!(reg.get("missing").is_none());
}

/// @covers: HandlerRegistry::list_ids
#[test]
fn test_list_ids_returns_registered_ids_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "b" }));
    reg.register(Arc::new(Fixed { id: "a" }));
    let mut ids = reg.list_ids();
    ids.sort();
    assert_eq!(ids, vec!["a", "b"]);
}

/// @covers: HandlerRegistry::list_ids — empty registry
#[test]
fn test_list_ids_empty_registry_returns_empty_edge() {
    let reg = make_reg();
    assert!(reg.list_ids().is_empty());
}

/// @covers: HandlerRegistry::len — counts correctly
#[test]
fn test_len_reflects_current_count_happy() {
    let reg = make_reg();
    assert_eq!(reg.len(), 0);
    reg.register(Arc::new(Fixed { id: "x" }));
    assert_eq!(reg.len(), 1);
    reg.deregister("x");
    assert_eq!(reg.len(), 0);
}

/// @covers: HandlerRegistry::is_empty — new registry
#[test]
fn test_is_empty_new_registry_returns_true_edge() {
    let reg = make_reg();
    assert!(reg.is_empty());
}

/// @covers: retrieved handler executes correctly
#[test]
fn test_retrieved_handler_executes_correctly_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "exec" }));
    let h = reg.get("exec").unwrap();
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext { security: &security, commands: &bus, observer: observer.as_ref() };
    assert_eq!(block_on(h.execute("data".into(), ctx)).unwrap(), "data");
}

/// @covers: HandlerRegistry::register — registering with a duplicate id does not produce an error
#[test]
fn test_register_duplicate_id_is_not_an_error_error() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "dup" }));
    reg.register(Arc::new(Fixed { id: "dup" }));
    assert_eq!(reg.len(), 1);
}

/// @covers: HandlerRegistry::deregister — re-deregistering already-removed id
#[test]
fn test_deregister_already_removed_id_returns_false_edge() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "once" }));
    assert!(reg.deregister("once"));
    assert!(!reg.deregister("once"));
}

/// @covers: HandlerRegistry::get — retrieve a known registered handler
#[test]
fn test_get_registered_handler_returns_some_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "present" }));
    let handler = reg.get("present");
    assert!(handler.is_some());
    assert_eq!(handler.unwrap().id(), "present");
}

/// @covers: HandlerRegistry::get — re-deregistered handler is gone
#[test]
fn test_get_after_deregister_returns_none_edge() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "gone" }));
    reg.deregister("gone");
    assert!(reg.get("gone").is_none());
}

/// @covers: HandlerRegistry::list_ids — no error path; empty registry returns empty vec
#[test]
fn test_list_ids_on_empty_registry_returns_empty_vec_error() {
    let reg = make_reg();
    assert!(reg.list_ids().is_empty());
}

/// @covers: HandlerRegistry::len — infallible; zero is valid, not an error
#[test]
fn test_len_zero_after_all_deregistered_is_not_error_error() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "ephemeral" }));
    reg.deregister("ephemeral");
    assert_eq!(reg.len(), 0);
}

/// @covers: HandlerRegistry::len — count after multiple registers equals distinct ids
#[test]
fn test_len_after_two_distinct_registers_is_two_edge() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "p" }));
    reg.register(Arc::new(Fixed { id: "q" }));
    assert_eq!(reg.len(), 2);
}

/// @covers: HandlerRegistry::is_empty — empty registry returns true
#[test]
fn test_is_empty_on_empty_registry_returns_true_happy() {
    let reg = make_reg();
    assert!(reg.is_empty());
}

/// @covers: HandlerRegistry::is_empty — non-empty registry never returns true as an error
#[test]
fn test_is_empty_after_register_returns_false_not_an_error_error() {
    let reg = make_reg();
    reg.register(Arc::new(Fixed { id: "item" }));
    assert!(!reg.is_empty());
}
