//! Integration tests — `InProcessHandlerRegistry` type.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError, HandlerRegistry, InProcessHandlerRegistry};
use edge_domain_security::SecurityContext;
use futures::executor::block_on;

struct Stub {
    id: &'static str,
    response: &'static str,
}

#[async_trait]
impl Handler for Stub {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str { self.id }
    async fn execute(&self, _req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        Ok(self.response.into())
    }
}

fn make_reg() -> InProcessHandlerRegistry<String, String> {
    InProcessHandlerRegistry::new()
}

/// @covers: InProcessHandlerRegistry::new — creates empty registry
#[test]
fn test_new_creates_empty_registry_happy() {
    let reg = make_reg();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}

/// @covers: InProcessHandlerRegistry default
#[test]
fn test_default_creates_empty_registry_edge() {
    let reg = InProcessHandlerRegistry::<String, String>::default();
    assert!(reg.is_empty());
}

/// @covers: InProcessHandlerRegistry::register — get returns Some
#[test]
fn test_register_makes_handler_retrievable_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Stub { id: "s1", response: "r1" }));
    assert!(reg.get("s1").is_some());
}

/// @covers: InProcessHandlerRegistry::register — duplicate id replaces
#[test]
fn test_register_duplicate_id_replaces_handler_edge() {
    let reg = make_reg();
    reg.register(Arc::new(Stub { id: "dup", response: "first" }));
    reg.register(Arc::new(Stub { id: "dup", response: "second" }));
    assert_eq!(reg.len(), 1);
    let h = reg.get("dup").unwrap();
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &bus };
    assert_eq!(block_on(h.execute("".into(), ctx)).unwrap(), "second");
}

/// @covers: InProcessHandlerRegistry::deregister — returns true for existing id
#[test]
fn test_deregister_existing_returns_true_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Stub { id: "to-remove", response: "x" }));
    assert!(reg.deregister("to-remove"));
    assert!(reg.get("to-remove").is_none());
}

/// @covers: InProcessHandlerRegistry::deregister — returns false for missing
#[test]
fn test_deregister_missing_returns_false_error() {
    let reg = make_reg();
    assert!(!reg.deregister("missing"));
}

/// @covers: InProcessHandlerRegistry::list_ids — sorted
#[test]
fn test_list_ids_returns_sorted_ids_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Stub { id: "z", response: "" }));
    reg.register(Arc::new(Stub { id: "a", response: "" }));
    reg.register(Arc::new(Stub { id: "m", response: "" }));
    let ids = reg.list_ids();
    assert_eq!(ids, vec!["a", "m", "z"]);
}

/// @covers: InProcessHandlerRegistry::list_ids — empty registry
#[test]
fn test_list_ids_empty_registry_returns_empty_vec_edge() {
    let reg = make_reg();
    assert!(reg.list_ids().is_empty());
}

/// @covers: retrieved handler executes correctly
#[test]
fn test_retrieved_handler_produces_expected_response_happy() {
    let reg = make_reg();
    reg.register(Arc::new(Stub { id: "exec", response: "pong" }));
    let h = reg.get("exec").unwrap();
    let security = SecurityContext::unauthenticated();
    let bus = StdCommandBusFactory::direct();
    let ctx = HandlerContext { security: &security, commands: &bus };
    assert_eq!(block_on(h.execute("ping".into(), ctx)).unwrap(), "pong");
}
