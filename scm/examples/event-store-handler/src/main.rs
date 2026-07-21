//! Runnable example: two `Handler`s sharing one constructor-injected `EventStore<Event>`, with
//! the per-call aggregate id and event payload carried through `Self::Request` — the
//! generic-per-type port wiring pattern `HandlerContext` structurally can't hold, since a single
//! context field can't be "an event store," only "an event store of `OrderCreated` events." See
//! issue #149.
//!
//! `RecordOrderCreatedHandler::execute` genuinely reads `HandlerContext` (it emits a log record
//! through `ctx.observer` on every call); the actual append doesn't need it — the `EventStore` it
//! holds is its own collaborator, injected once at construction, same shape as
//! `SaveOrderHandler`'s `Repository` in `examples/repository-handler`. Both handlers here share
//! the same injected `Arc<dyn EventStore<Event = OrderCreated>>` instance.
//!
//! Run with: `cargo run -p edge-application-event-store-handler-example`

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError, EventStore,
    EventStoreAppendRequest, EventStoreLoadRequest, EventTypeRequest, EventTypeResponse,
    ExpectedVersion, MemoryEventStore,
};
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;

#[derive(Debug, Clone)]
struct OrderCreated {
    order_id: String,
    item: String,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "order.created",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.order_id,
        })
    }
}

#[derive(Debug, Clone)]
struct RecordOrderCreatedRequest {
    order_id: String,
    item: String,
}
impl edge_application_base::Request for RecordOrderCreatedRequest {}

#[derive(Debug, Clone, Copy)]
struct RecordOrderCreatedResponse {
    sequence: u64,
}
impl edge_application_base::Response for RecordOrderCreatedResponse {}

/// Holds its own injected `EventStore`; genuinely reads `req.ctx` inside `execute()`.
struct RecordOrderCreatedHandler {
    event_store: Arc<dyn EventStore<Event = OrderCreated>>,
}

#[async_trait]
impl Handler for RecordOrderCreatedHandler {
    type Request = RecordOrderCreatedRequest;
    type Response = RecordOrderCreatedResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, RecordOrderCreatedRequest>,
    ) -> Result<RecordOrderCreatedResponse, HandlerError> {
        // ctx IS genuinely read here — real per-request observability, not filler.
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "record_order_created_handler".to_string(),
                message: format!("appending order.created for {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        println!(
            "  [1] RecordOrderCreatedHandler::execute — delegating to its own injected EventStore"
        );
        let result = self
            .event_store
            .append(EventStoreAppendRequest {
                aggregate_id: &req.req.order_id,
                events: vec![OrderCreated {
                    order_id: req.req.order_id.clone(),
                    item: req.req.item,
                }],
                expected: ExpectedVersion::Any,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        println!(
            "  [2] RecordOrderCreatedHandler::execute — event store confirmed sequence {}",
            result.sequence
        );
        Ok(RecordOrderCreatedResponse {
            sequence: result.sequence,
        })
    }
}

#[derive(Debug, Clone)]
struct GetOrderHistoryRequest {
    order_id: String,
}
impl edge_application_base::Request for GetOrderHistoryRequest {}

#[derive(Debug, Clone)]
struct GetOrderHistoryResponse {
    items: Vec<String>,
}
impl edge_application_base::Response for GetOrderHistoryResponse {}

/// Holds the *same* injected `EventStore` instance as `RecordOrderCreatedHandler` — one event
/// store, multiple handlers, each reaching it through their own constructor-injected field.
struct GetOrderHistoryHandler {
    event_store: Arc<dyn EventStore<Event = OrderCreated>>,
}

#[async_trait]
impl Handler for GetOrderHistoryHandler {
    type Request = GetOrderHistoryRequest;
    type Response = GetOrderHistoryResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, GetOrderHistoryRequest>,
    ) -> Result<GetOrderHistoryResponse, HandlerError> {
        println!(
            "  [1] GetOrderHistoryHandler::execute — delegating to its own injected EventStore"
        );
        let loaded = self
            .event_store
            .load(EventStoreLoadRequest {
                aggregate_id: &req.req.order_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        let items: Vec<String> = loaded
            .events
            .iter()
            .map(|envelope| format!("seq {}: {}", envelope.sequence, envelope.event.item))
            .collect();
        println!("  [2] GetOrderHistoryHandler::execute — event store returned {items:?}");
        Ok(GetOrderHistoryResponse { items })
    }
}

#[tokio::main]
async fn main() {
    println!("=== Handler + injected EventStore<Event> — constructor injection, Self::Request carries the aggregate id/payload ===\n");

    let event_store: Arc<dyn EventStore<Event = OrderCreated>> =
        Arc::new(MemoryEventStore::<OrderCreated>::new());
    let record_handler = RecordOrderCreatedHandler {
        event_store: Arc::clone(&event_store),
    };
    let history_handler = GetOrderHistoryHandler {
        event_store: Arc::clone(&event_store),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] record_handler.execute(RecordOrderCreatedRequest {{ order_id: \"order-1\", item: \"widget\" }})");
    let record_resp = record_handler
        .execute(HandlerExecutionRequest {
            req: RecordOrderCreatedRequest {
                order_id: "order-1".to_string(),
                item: "widget".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("append should succeed");
    println!(
        "[3] RecordOrderCreatedHandler reports sequence = {}\n",
        record_resp.sequence
    );

    println!("[0] record_handler.execute(RecordOrderCreatedRequest {{ order_id: \"order-1\", item: \"gadget\" }})");
    let record_resp_2 = record_handler
        .execute(HandlerExecutionRequest {
            req: RecordOrderCreatedRequest {
                order_id: "order-1".to_string(),
                item: "gadget".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("append should succeed");
    println!(
        "[3] RecordOrderCreatedHandler reports sequence = {}\n",
        record_resp_2.sequence
    );

    println!("[0] history_handler.execute(GetOrderHistoryRequest {{ order_id: \"order-1\" }})");
    let history_resp = history_handler
        .execute(HandlerExecutionRequest {
            req: GetOrderHistoryRequest {
                order_id: "order-1".to_string(),
            },
            ctx: &ctx,
        })
        .await
        .expect("load should succeed");
    println!(
        "[3] GetOrderHistoryHandler returned {:?}\n",
        history_resp.items
    );

    println!("Conclusion: RecordOrderCreatedHandler used ctx.observer for real per-request logging,");
    println!("but both handlers reached the actual event stream only through their own,");
    println!("independently injected EventStore<OrderCreated> — never through HandlerContext, which");
    println!("structurally cannot hold a generic-per-type port. Swap MemoryEventStore for a real");
    println!("database-backed EventStore impl and neither handler changes.");
}
