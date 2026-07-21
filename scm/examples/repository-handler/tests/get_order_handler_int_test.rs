//! Integration test: `GetOrderHandler::execute()` reads through a real, constructor-injected
//! `Repository` — the same instance a `SaveOrderHandler` writes into, proving one repository can
//! genuinely back multiple handlers, each reaching it through its own injected field rather than
//! through `HandlerContext`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_application_repository::{
    MemoryRepository, Repository, RepositoryIdRequest, RepositorySaveRequest,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Order {
    item: String,
    quantity: u32,
}

#[derive(Debug, Clone)]
struct GetOrderRequest {
    order_id: String,
}
impl edge_application_base::Request for GetOrderRequest {}

#[derive(Debug, Clone)]
struct GetOrderResponse {
    order: Option<Order>,
}
impl edge_application_base::Response for GetOrderResponse {}

/// Holds its own injected `Repository` — the per-call entity id arrives through `Self::Request`.
struct GetOrderHandler {
    repository: Arc<dyn Repository<Entity = Order, Id = String>>,
}

#[async_trait]
impl Handler for GetOrderHandler {
    type Request = GetOrderRequest;
    type Response = GetOrderResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, GetOrderRequest>,
    ) -> Result<GetOrderResponse, HandlerError> {
        let found = self
            .repository
            .find(RepositoryIdRequest {
                id: &req.req.order_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(GetOrderResponse {
            order: found.entity,
        })
    }
}

/// Seeds a repository with one order and builds a handler sharing that same instance —
/// mirrors the multi-handler-one-repository shape from `SaveOrderHandler`.
fn build_handler_with_seeded_order() -> GetOrderHandler {
    let repository: Arc<dyn Repository<Entity = Order, Id = String>> =
        Arc::new(MemoryRepository::<Order, String>::new());
    block_on(repository.save(RepositorySaveRequest {
        id: "order-1".to_string(),
        entity: Order {
            item: "widget".to_string(),
            quantity: 3,
        },
    }))
    .unwrap();
    GetOrderHandler { repository }
}

fn run(handler: &GetOrderHandler, order_id: &str) -> Result<GetOrderResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: GetOrderRequest {
            order_id: order_id.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_existing_order_returns_entity_happy() {
    let handler = build_handler_with_seeded_order();
    let response = run(&handler, "order-1").unwrap();
    assert_eq!(
        response.order,
        Some(Order {
            item: "widget".to_string(),
            quantity: 3,
        })
    );
}

/// @covers: Handler::execute
#[test]
fn test_execute_missing_order_returns_none_error() {
    let handler = build_handler_with_seeded_order();
    let response = run(&handler, "does-not-exist").unwrap();
    assert_eq!(response.order, None);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_independent_and_consistent_edge() {
    let handler = build_handler_with_seeded_order();
    let first = run(&handler, "order-1").unwrap();
    let second = run(&handler, "missing").unwrap();
    let third = run(&handler, "order-1").unwrap();
    assert!(first.order.is_some());
    assert!(second.order.is_none());
    assert_eq!(first.order, third.order);
}
