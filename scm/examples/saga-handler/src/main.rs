//! Runnable example: two `Handler`s sharing one constructor-injected saga instance, with the
//! per-call event data carried through `Self::Request` — the generic-per-type port wiring
//! pattern `HandlerContext` structurally can't hold. See issue #149.
//!
//! **A real structural finding, not a workaround:** `SagaStore::get<'a>(&'a self, ...) ->
//! Result<SagaGetResponse<'a, Self::SagaInstance>, SagaError>` returns an *immutable* borrow of
//! the stored saga, but `Saga::handle(&mut self, ...)` requires a *mutable* one. No amount of
//! external locking around the store fixes this — the trait method's own return type is the
//! blocker, not the caller's access pattern. `SagaStore` as currently shaped can register and
//! look sagas up (both fine, `is_complete` only needs `&self` too), but it cannot support
//! "retrieve a saga, then call `handle` on it" at all. So this example wires `Handler` directly
//! to a single `Saga` instance behind `Arc<parking_lot::Mutex<S>>` — bypassing `SagaStore`
//! entirely for the mutation path, because `SagaStore` structurally can't do it, not because
//! this example chose to skip it.
//!
//! `HandleSagaEventHandler::execute` genuinely reads `HandlerContext` for logging; the saga
//! mutation itself doesn't need it. Both handlers here share the same injected
//! `Arc<Mutex<OrderSaga>>` instance.
//!
//! Run with: `cargo run -p edge-application-saga-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_saga::{
    Saga, SagaCommand, SagaCommandDispatchRequest, SagaError, SagaEvent,
    SagaEventDescribeRequest, SagaEventDescribeResponse, SagaHandleRequest, SagaHandleResponse,
    SagaIsCompleteRequest, SagaIsCompleteResponse,
};
use edge_security_runtime::SecurityContext;
use parking_lot::Mutex;

#[derive(Debug, Clone)]
struct OrderPaid {
    order_id: String,
}

impl SagaEvent for OrderPaid {
    fn describe(
        &self,
        _req: SagaEventDescribeRequest,
    ) -> Result<SagaEventDescribeResponse, SagaError> {
        Ok(SagaEventDescribeResponse {
            event_type: "order.paid".to_string(),
            aggregate_id: self.order_id.clone(),
        })
    }
}

#[derive(Debug, Clone)]
struct ShipOrderCommand {
    order_id: String,
}

impl SagaCommand for ShipOrderCommand {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), SagaError>> {
        let order_id = self.order_id.clone();
        Box::pin(async move {
            println!("      [infra] ShipOrderCommand::dispatch — shipping {order_id:?}");
            Ok(())
        })
    }
}

#[derive(Debug, Default)]
struct OrderSaga {
    shipped: bool,
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = OrderPaid;
    type Command = ShipOrderCommand;

    fn handle(
        &mut self,
        req: SagaHandleRequest<'_, OrderPaid>,
    ) -> Result<SagaHandleResponse<ShipOrderCommand>, SagaError> {
        self.shipped = true;
        Ok(SagaHandleResponse {
            commands: vec![ShipOrderCommand {
                order_id: req.event.order_id.clone(),
            }],
        })
    }

    fn is_complete(&self, _req: SagaIsCompleteRequest) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.shipped,
        })
    }
}

#[derive(Debug, Clone)]
struct HandleSagaEventRequest {
    order_id: String,
}
impl edge_application_base::Request for HandleSagaEventRequest {}

#[derive(Debug, Clone, Copy)]
struct HandleSagaEventResponse {
    commands_dispatched: usize,
}
impl edge_application_base::Response for HandleSagaEventResponse {}

/// Holds its own injected saga behind a lock — the only way to reach `Saga::handle`'s `&mut
/// self`, since `SagaStore::get` can't provide mutable access. Genuinely reads `req.ctx`.
struct HandleSagaEventHandler {
    saga: Arc<Mutex<OrderSaga>>,
}

#[async_trait]
impl Handler for HandleSagaEventHandler {
    type Request = HandleSagaEventRequest;
    type Response = HandleSagaEventResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, HandleSagaEventRequest>,
    ) -> Result<HandleSagaEventResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "handle_saga_event_handler".to_string(),
                message: format!("handling order.paid for {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!("  [1] HandleSagaEventHandler::execute — delegating to its own injected saga");
        let event = OrderPaid {
            order_id: req.req.order_id,
        };
        let commands = {
            let mut saga = self.saga.lock();
            saga.handle(SagaHandleRequest { event: &event })
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
                .commands
        };
        println!(
            "  [2] HandleSagaEventHandler::execute — saga staged {} command(s)",
            commands.len()
        );
        for command in &commands {
            command
                .dispatch(SagaCommandDispatchRequest)
                .await
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        }
        Ok(HandleSagaEventResponse {
            commands_dispatched: commands.len(),
        })
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct CheckSagaCompleteRequest;
impl edge_application_base::Request for CheckSagaCompleteRequest {}

#[derive(Debug, Clone, Copy)]
struct CheckSagaCompleteResponse {
    complete: bool,
}
impl edge_application_base::Response for CheckSagaCompleteResponse {}

/// Holds the *same* injected saga instance as `HandleSagaEventHandler`.
struct CheckSagaCompleteHandler {
    saga: Arc<Mutex<OrderSaga>>,
}

#[async_trait]
impl Handler for CheckSagaCompleteHandler {
    type Request = CheckSagaCompleteRequest;
    type Response = CheckSagaCompleteResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, CheckSagaCompleteRequest>,
    ) -> Result<CheckSagaCompleteResponse, HandlerError> {
        println!("  [1] CheckSagaCompleteHandler::execute — delegating to its own injected saga");
        let complete = self
            .saga
            .lock()
            .is_complete(SagaIsCompleteRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .complete;
        println!("  [2] CheckSagaCompleteHandler::execute — saga reports complete = {complete}");
        Ok(CheckSagaCompleteResponse { complete })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected Saga (Arc<Mutex<S>>) — constructor injection, Self::Request carries the event ===\n");

    let saga = Arc::new(Mutex::new(OrderSaga::default()));
    let handle_handler = HandleSagaEventHandler {
        saga: Arc::clone(&saga),
    };
    let check_handler = CheckSagaCompleteHandler {
        saga: Arc::clone(&saga),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] check_handler.execute(CheckSagaCompleteRequest) — before any event");
    let before = check_handler
        .execute(HandlerExecutionRequest {
            req: CheckSagaCompleteRequest,
            ctx: &ctx,
        })
        .await
        .expect("is_complete should succeed");
    println!("[3] CheckSagaCompleteHandler reports complete = {}\n", before.complete);

    println!("[0] handle_handler.execute(HandleSagaEventRequest {{ order_id: \"order-1\" }})");
    let handled = handle_handler
        .execute(HandlerExecutionRequest {
            req: HandleSagaEventRequest {
                order_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("handle should succeed");
    println!(
        "[3] HandleSagaEventHandler dispatched {} command(s)\n",
        handled.commands_dispatched
    );

    println!("[0] check_handler.execute(CheckSagaCompleteRequest) — after the event");
    let after = check_handler
        .execute(HandlerExecutionRequest {
            req: CheckSagaCompleteRequest,
            ctx: &ctx,
        })
        .await
        .expect("is_complete should succeed");
    println!("[3] CheckSagaCompleteHandler reports complete = {}\n", after.complete);

    println!("Conclusion: both handlers reached the same saga only through their own,");
    println!("independently injected Arc<Mutex<OrderSaga>> — never through HandlerContext. This");
    println!("bypasses SagaStore entirely, because SagaStore::get()'s immutable-borrow return type");
    println!("cannot support the get-then-handle mutation path Saga::handle requires.");
}
