#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! Tests for InProcessHandlerRegistry via the public Domain factory.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::Domain;
use edge_domain::Handler;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;

struct ConstHandler {
    id: &'static str,
    response: i32,
}

#[async_trait]
impl Handler for ConstHandler {
    type Request = i32;
    type Response = i32;
    fn id(&self) -> &str {
        self.id
    }
    fn pattern(&self) -> &str {
        "*"
    }
    async fn execute(&self, _: i32, _ctx: HandlerContext<'_>) -> Result<i32, HandlerError> {
        Ok(self.response)
    }
}

fn make_handler(id: &'static str) -> Arc<dyn Handler<Request = i32, Response = i32>> {
    Arc::new(ConstHandler { id, response: 0 })
}

#[test]
fn test_in_process_handler_registry_new_is_empty() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert_eq!(reg.len(), 0);
    assert!(reg.list_ids().is_empty());
}

#[test]
fn test_in_process_handler_registry_register_makes_handler_retrievable() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(make_handler("a"));
    assert!(reg.get("a").is_some());
}

#[test]
fn test_in_process_handler_registry_get_returns_none_for_unknown_id() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert!(reg.get("missing").is_none());
}

#[test]
fn test_in_process_handler_registry_deregister_removes_and_returns_true() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(make_handler("x"));
    assert!(reg.deregister("x"));
    assert!(reg.get("x").is_none());
}

#[test]
fn test_in_process_handler_registry_deregister_missing_returns_false() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert!(!reg.deregister("ghost"));
}

#[test]
fn test_in_process_handler_registry_register_same_id_replaces_previous() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(make_handler("dup"));
    reg.register(make_handler("dup"));
    assert_eq!(reg.len(), 1);
}

#[test]
fn test_in_process_handler_registry_list_ids_returns_all_registered() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(make_handler("p"));
    reg.register(make_handler("q"));
    let mut ids = reg.list_ids();
    ids.sort();
    assert_eq!(ids, vec!["p", "q"]);
}

#[test]
fn test_in_process_handler_registry_len_reflects_count() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert_eq!(reg.len(), 0);
    reg.register(make_handler("one"));
    assert_eq!(reg.len(), 1);
    reg.register(make_handler("two"));
    assert_eq!(reg.len(), 2);
}
