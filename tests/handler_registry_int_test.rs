//! Integration tests for `HandlerRegistry`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Handler, HandlerError, HandlerRegistry, RequestContext};

struct EchoHandler { id: String }

#[async_trait]
impl Handler<String, String> for EchoHandler {
    fn id(&self) -> &str { &self.id }
    fn pattern(&self) -> &str { "echo" }
    async fn execute(&self, req: String) -> Result<String, HandlerError> { Ok(req) }
}

fn echo(id: &str) -> Arc<dyn Handler<String, String>> {
    Arc::new(EchoHandler { id: id.to_string() })
}

/// @covers: HandlerRegistry::new
#[test]
fn test_handler_registry_struct_new_creates_empty_registry() {
    let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
    assert!(reg.is_empty());
}

/// @covers: HandlerRegistry::register
#[test]
fn test_handler_registry_struct_register_makes_handler_retrievable() {
    let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
    reg.register(echo("svc"));
    assert!(reg.get("svc").is_some());
}

/// @covers: HandlerRegistry::deregister
#[test]
fn test_handler_registry_struct_deregister_removes_handler() {
    let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
    reg.register(echo("svc"));
    assert!(reg.deregister("svc"));
    assert!(reg.get("svc").is_none());
}

/// @covers: HandlerRegistry::list_ids
#[test]
fn test_handler_registry_struct_list_ids_returns_all_ids() {
    let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
    reg.register(echo("a"));
    reg.register(echo("b"));
    let mut ids = reg.list_ids();
    ids.sort();
    assert_eq!(ids, vec!["a", "b"]);
}

/// @covers: HandlerRegistry::len
#[test]
fn test_handler_registry_struct_len_reflects_registration_count() {
    let reg: HandlerRegistry<String, String> = HandlerRegistry::new();
    assert_eq!(reg.len(), 0);
    reg.register(echo("a"));
    assert_eq!(reg.len(), 1);
}
