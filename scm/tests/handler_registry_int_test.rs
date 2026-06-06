//! Integration tests for `HandlerRegistry`.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::{Dispatch, Handler, HandlerError, HandlerRegistry};

struct EchoHandler {
    id: String,
}

#[async_trait]
impl Handler<String, String> for EchoHandler {
    fn id(&self) -> &str {
        &self.id
    }
    fn pattern(&self) -> &str {
        "echo"
    }
    async fn execute(&self, req: String) -> Result<String, HandlerError> {
        Ok(req)
    }
}

fn echo(id: &str) -> Arc<dyn Handler<String, String>> {
    Arc::new(EchoHandler { id: id.to_string() })
}

fn registry() -> Arc<dyn HandlerRegistry<String, String>> {
    Dispatch::new_handler_registry()
}

/// @covers: HandlerRegistry — starts empty
#[test]
fn test_new_registry_is_empty() {
    let reg = registry();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}

/// @covers: HandlerRegistry::register
#[test]
fn test_register_makes_handler_retrievable() {
    let reg = registry();
    reg.register(echo("svc"));
    assert!(reg.get("svc").is_some());
}

/// @covers: HandlerRegistry::deregister
#[test]
fn test_deregister_removes_handler_and_returns_true() {
    let reg = registry();
    reg.register(echo("svc"));
    assert!(reg.deregister("svc"));
    assert!(reg.get("svc").is_none());
}

/// @covers: HandlerRegistry::deregister — missing id returns false
#[test]
fn test_deregister_missing_handler_returns_false() {
    let reg = registry();
    assert!(!reg.deregister("ghost"));
}

/// @covers: HandlerRegistry::list_ids
#[test]
fn test_list_ids_returns_all_registered_ids() {
    let reg = registry();
    reg.register(echo("a"));
    reg.register(echo("b"));
    let mut ids = reg.list_ids();
    ids.sort();
    assert_eq!(ids, vec!["a", "b"]);
}

/// @covers: HandlerRegistry::len
#[test]
fn test_len_reflects_registration_count() {
    let reg = registry();
    assert_eq!(reg.len(), 0);
    reg.register(echo("a"));
    assert_eq!(reg.len(), 1);
    reg.register(echo("b"));
    assert_eq!(reg.len(), 2);
}

/// @covers: HandlerRegistry — registering same id replaces entry
#[test]
fn test_register_same_id_replaces_existing() {
    let reg = registry();
    reg.register(echo("svc"));
    reg.register(echo("svc"));
    assert_eq!(reg.len(), 1);
}
