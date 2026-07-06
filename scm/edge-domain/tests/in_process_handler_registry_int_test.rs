#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! Tests for InProcessHandlerRegistry via the public Domain factory.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain::Domain;
use edge_domain::Handler;
use edge_domain::HandlerContext;
use edge_domain::HandlerError;
use edge_domain_handler::{
    DeregisterHandlerRequest, ExecutionRequest, HandlerLookupRequest, IdRequest, IdResponse,
    LenRequest, ListIdsRequest, RegisterHandlerRequest,
};

struct ConstHandler {
    id: &'static str,
    response: i32,
}

#[async_trait]
impl Handler for ConstHandler {
    type Request = i32;
    type Response = i32;
    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: self.id.to_string(),
        })
    }
    async fn execute(&self, _req: ExecutionRequest<'_, i32>) -> Result<i32, HandlerError> {
        Ok(self.response)
    }
}

fn make_handler(id: &'static str) -> Arc<dyn Handler<Request = i32, Response = i32>> {
    Arc::new(ConstHandler { id, response: 0 })
}

#[test]
fn test_in_process_handler_registry_new_is_empty() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    assert!(reg.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

#[test]
fn test_in_process_handler_registry_register_makes_handler_retrievable() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(RegisterHandlerRequest::new(make_handler("a")))
        .unwrap();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "a".to_string()
        })
        .unwrap()
        .handler
        .is_some());
}

#[test]
fn test_in_process_handler_registry_get_returns_none_for_unknown_id() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "missing".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}

#[test]
fn test_in_process_handler_registry_deregister_removes_and_returns_true() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(RegisterHandlerRequest::new(make_handler("x")))
        .unwrap();
    assert!(
        reg.deregister(DeregisterHandlerRequest {
            id: "x".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(reg
        .get(HandlerLookupRequest {
            id: "x".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}

#[test]
fn test_in_process_handler_registry_deregister_missing_returns_false() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert!(
        !reg.deregister(DeregisterHandlerRequest {
            id: "ghost".to_string()
        })
        .unwrap()
        .was_present
    );
}

#[test]
fn test_in_process_handler_registry_register_same_id_replaces_previous() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(RegisterHandlerRequest::new(make_handler("dup")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(make_handler("dup")))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
}

#[test]
fn test_in_process_handler_registry_list_ids_returns_all_registered() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    reg.register(RegisterHandlerRequest::new(make_handler("p")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(make_handler("q")))
        .unwrap();
    let mut ids = reg.list_ids(ListIdsRequest).unwrap().ids;
    ids.sort();
    assert_eq!(ids, vec!["p", "q"]);
}

#[test]
fn test_in_process_handler_registry_len_reflects_count() {
    let reg = Domain::new_handler_registry::<i32, i32>();
    assert_eq!(reg.len(LenRequest).unwrap().count, 0);
    reg.register(RegisterHandlerRequest::new(make_handler("one")))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 1);
    reg.register(RegisterHandlerRequest::new(make_handler("two")))
        .unwrap();
    assert_eq!(reg.len(LenRequest).unwrap().count, 2);
}
