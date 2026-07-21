//! Runnable example: two `Handler`s sharing one constructor-injected `Repository<Entity, Id>`,
//! with the per-call entity id carried through `Self::Request` — the generic-per-type port
//! wiring pattern `HandlerContext` structurally can't hold, since a single context field can't
//! be "a repository," only "a repository of `Order`s." See issue #149.
//!
//! `SaveOrderHandler::execute` genuinely reads `HandlerContext` (it emits a log record through
//! `ctx.observer` on every call); the actual persistence doesn't need it — the `Repository` it
//! holds is its own collaborator, injected once at construction, exactly like `AuthHandler`'s
//! `QueryBus` in `examples/service-query`. Both handlers here share the same injected
//! `Arc<dyn Repository<...>>` instance, demonstrating the realistic case where one repository
//! backs multiple handlers.
//!
//! Run with: `cargo run -p edge-application-repository-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_repository::{
    MemoryRepository, Repository, RepositoryIdRequest, RepositorySaveRequest,
};
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Order {
    item: String,
    quantity: u32,
}

#[derive(Debug, Clone)]
struct SaveOrderRequest {
    order_id: String,
    item: String,
    quantity: u32,
}
impl edge_application_base::Request for SaveOrderRequest {}

#[derive(Debug, Clone, Copy)]
struct SaveOrderResponse {
    saved: bool,
}
impl edge_application_base::Response for SaveOrderResponse {}

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

/// Holds its own injected `Repository`; genuinely reads `req.ctx` inside `execute()`.
struct SaveOrderHandler {
    repository: Arc<dyn Repository<Entity = Order, Id = String>>,
}

#[async_trait]
impl Handler for SaveOrderHandler {
    type Request = SaveOrderRequest;
    type Response = SaveOrderResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SaveOrderRequest>,
    ) -> Result<SaveOrderResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "save_order_handler".to_string(),
                message: format!("saving order {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!("  [1] SaveOrderHandler::execute — delegating to its own injected Repository");
        self.repository
            .save(RepositorySaveRequest {
                id: req.req.order_id,
                entity: Order {
                    item: req.req.item,
                    quantity: req.req.quantity,
                },
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        println!("  [2] SaveOrderHandler::execute — repository confirmed the save");
        Ok(SaveOrderResponse { saved: true })
    }
}

/// Holds the *same* injected `Repository` instance as `SaveOrderHandler` — one repository,
/// multiple handlers, each reaching it through their own constructor-injected field.
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
        println!("  [1] GetOrderHandler::execute — delegating to its own injected Repository");
        let found = self
            .repository
            .find(RepositoryIdRequest {
                id: &req.req.order_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        println!(
            "  [2] GetOrderHandler::execute — repository returned {:?}",
            found.entity
        );
        Ok(GetOrderResponse {
            order: found.entity,
        })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected Repository<Entity, Id> — constructor injection, Self::Request carries the id ===\n");

    let repository: Arc<dyn Repository<Entity = Order, Id = String>> =
        Arc::new(MemoryRepository::<Order, String>::new());
    let save_handler = SaveOrderHandler {
        repository: Arc::clone(&repository),
    };
    let get_handler = GetOrderHandler {
        repository: Arc::clone(&repository),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] save_handler.execute(SaveOrderRequest {{ order_id: \"order-1\", item: \"widget\", quantity: 3 }})");
    let save_resp = save_handler
        .execute(HandlerExecutionRequest {
            req: SaveOrderRequest {
                order_id: "order-1".to_string(),
                item: "widget".to_string(),
                quantity: 3,
            },
            ctx: &ctx,
        })
        .await
        .expect("save should succeed");
    println!("[3] SaveOrderHandler reports saved = {}\n", save_resp.saved);

    println!("[0] get_handler.execute(GetOrderRequest {{ order_id: \"order-1\" }})");
    let get_resp = get_handler
        .execute(HandlerExecutionRequest {
            req: GetOrderRequest {
                order_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("get should succeed");
    println!("[3] GetOrderHandler returned {:?}\n", get_resp.order);

    println!("[0] get_handler.execute(GetOrderRequest {{ order_id: \"missing\" }})");
    let missing_resp = get_handler
        .execute(HandlerExecutionRequest {
            req: GetOrderRequest {
                order_id: "missing".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("get should succeed even on a miss");
    println!("[3] GetOrderHandler returned {:?}\n", missing_resp.order);

    println!("Conclusion: SaveOrderHandler used ctx.observer for real per-request logging, but");
    println!("both handlers reached the actual store only through their own, independently");
    println!("injected Repository<Order, String> — never through HandlerContext, which structurally");
    println!("cannot hold a generic-per-type port. Swap MemoryRepository for a real database-backed");
    println!("Repository impl and neither handler changes.");
}
