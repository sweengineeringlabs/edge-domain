# edge-domain

L2 Domain contract for the `swe-edge` framework.

Defines the business logic contracts consumed by ingress, egress, and the runtime.
No knowledge of transport protocols, databases, or messaging infrastructure.

## Contracts

| Type | Purpose |
|------|---------|
| `Handler<Req, Resp>` | Ingress-facing execution unit — receives a request, returns a response |
| `Service<Req, Resp>` | Domain operation — called by handlers, other services, or background jobs |
| `Repository<T, Id>` | Data access — find, save, delete, list entities |
| `QueryableRepository<T, Id>` | Spec-based queries — `find_by`, `find_one_by`, `count_by` |
| `DomainEvent` | Immutable fact that something happened |
| `EventPublisher` | Emits domain events to subscribers |
| `Aggregate` | State rebuilt entirely by replaying a sequence of domain events |
| `EventStore<E>` | Append-only event stream store — `append`, `load`, `load_from` |
| `EventEnvelope<E>` | Domain event with store metadata (sequence, aggregate_id, occurred_at) |
| `ExpectedVersion` | Optimistic concurrency guard — `Any`, `NoStream`, `Exact(u64)` |
| `Command` | Write operation — mutates state, returns `()` |
| `Query<R>` | Read operation — returns data, never mutates |
| `CommandBus` | Dispatches commands |
| `QueryBus<R>` | Dispatches queries |

## Application bootstrap pattern

Wire concrete implementations into handlers at startup. `edge-domain` owns the
traits; your application crate (or an infrastructure crate) supplies the implementations.

```rust
use std::sync::Arc;
use async_trait::async_trait;
use edge_domain::{
    Handler, HandlerError, HandlerRegistry, Repository, Service, ServiceError,
    EventPublisher, new_in_memory_repository, noop_event_publisher, new_handler_registry,
};

// 1. Define your domain types
struct CreateOrderReq { customer_id: String }
struct OrderId(String);

// 2. Implement Service (business logic)
struct CreateOrderService {
    repo:      Arc<dyn Repository<OrderId, String>>,
    publisher: Arc<dyn EventPublisher>,
}

#[async_trait]
impl Service<CreateOrderReq, OrderId> for CreateOrderService {
    fn name(&self) -> &str { "create-order" }
    async fn execute(&self, req: CreateOrderReq) -> Result<OrderId, ServiceError> {
        let id = OrderId(uuid::Uuid::new_v4().to_string());
        self.repo.save(id.0.clone(), id.0.clone()).await
            .map_err(|e| ServiceError::Internal(e.to_string()))?;
        Ok(id)
    }
}

// 3. Implement Handler (ingress adapter over the service)
struct CreateOrderHandler {
    service: Arc<dyn Service<CreateOrderReq, OrderId>>,
}

#[async_trait]
impl Handler<CreateOrderReq, OrderId> for CreateOrderHandler {
    fn id(&self)      -> &str { "create-order" }
    fn pattern(&self) -> &str { "/orders" }
    async fn execute(&self, req: CreateOrderReq) -> Result<OrderId, HandlerError> {
        self.service.execute(req).await?  // ServiceError → HandlerError via From
    }
}

// 4. Bootstrap — wire concrete impls into the handler, register with the runtime
#[tokio::main]
async fn main() {
    let repo      = new_in_memory_repository::<String, String>();
    let publisher = noop_event_publisher();

    let service = Arc::new(CreateOrderService { repo, publisher });
    let handler = Arc::new(CreateOrderHandler { service });

    let registry: Arc<HandlerRegistry<CreateOrderReq, OrderId>> = new_handler_registry();
    registry.register(handler);

    // Pass registry to the runtime ...
}
```

## Error bridging

All domain error types convert to `HandlerError` via `?`:

```rust
async fn execute(&self, req: Req) -> Result<Resp, HandlerError> {
    let item = self.repo.find(&req.id).await?;    // RepositoryError → HandlerError
    let resp = self.service.execute(req).await?;  // ServiceError    → HandlerError
    self.bus.dispatch(Box::new(cmd)).await?;       // CommandError    → HandlerError
    self.verifier.verify(&token)?;                 // VerifierError   → HandlerError
    Ok(resp)
}
```

## Error types

| Error | Variants |
|-------|---------|
| `HandlerError` | `InvalidRequest`, `NotFound`, `Conflict`, `ExecutionFailed`, `Unsupported`, `Unhealthy`, `FailedPrecondition`, `Unauthorized`, `PermissionDenied` |
| `ServiceError` | `InvalidRequest`, `RuleViolation`, `NotFound`, `Unavailable`, `Internal` |
| `RepositoryError` | `NotFound`, `Conflict`, `Unavailable`, `Internal` |
| `CommandError` | `InvalidInput`, `RuleViolation`, `NotFound`, `Internal` |
| `QueryError` | `InvalidInput`, `NotFound`, `Internal` |
| `EventError` | `SerializationFailed`, `Unavailable` |

`HandlerError::Unauthorized` maps to HTTP 401 / gRPC `UNAUTHENTICATED`.
`HandlerError::PermissionDenied` maps to HTTP 403 / gRPC `PERMISSION_DENIED`.

## Event sourcing

`edge-domain` provides provider-agnostic event sourcing contracts. Persistence backends
(EventStoreDB, PostgreSQL, etc.) implement `EventStore<E>` in infrastructure crates.

```rust
use edge_domain::{
    new_in_memory_event_store, reconstitute,
    Aggregate, DomainEvent, ExpectedVersion,
};
use std::time::SystemTime;

// 1. Define your event
#[derive(Clone)]
struct OrderPlaced { order_id: String }

impl DomainEvent for OrderPlaced {
    fn event_type(&self) -> &str { "order.placed" }
    fn aggregate_id(&self) -> &str { &self.order_id }
    fn occurred_at(&self) -> SystemTime { SystemTime::now() }
}

// 2. Define your aggregate
#[derive(Default)]
struct Order { id: String, placed: bool }

impl Aggregate for Order {
    type Event = OrderPlaced;
    fn apply(&mut self, event: &OrderPlaced) {
        self.id = event.order_id.clone();
        self.placed = true;
    }
    fn id(&self) -> &str { &self.id }
}

// 3. Append events and reconstitute
let store = new_in_memory_event_store::<OrderPlaced>();
store.append("order-1", vec![OrderPlaced { order_id: "order-1".into() }],
             ExpectedVersion::NoStream).await?;

let order = reconstitute::<Order>(&*store, "order-1").await?
    .expect("aggregate exists");
assert!(order.placed);
```

`ExpectedVersion` provides optimistic concurrency control:
- `Any` — always appends
- `NoStream` — rejects if stream already exists
- `Exact(n)` — rejects if stream version ≠ n (concurrent-write protection)

## Default implementations

Provided for development and testing — swap with real infrastructure in production:

| Factory | Returns | Use case |
|---------|---------|---------|
| `new_in_memory_repository::<T, Id>()` | `Arc<dyn Repository<T, Id>>` | Tests, local dev |
| `new_in_memory_queryable_repository::<T, Id>()` | `Arc<dyn QueryableRepository<T, Id>>` | Tests with spec-based queries |
| `new_in_memory_event_store::<E>()` | `Arc<dyn EventStore<E>>` | Tests, local dev |
| `reconstitute::<A>(store, id)` | `Result<Option<A>, EventStoreError>` | Replay events into aggregate |
| `echo_handler(id, pattern)` | `Arc<dyn Handler<T, T>>` | Transport-layer tests (returns input unchanged) |
| `direct_command_bus()` | `Arc<dyn CommandBus>` | In-process command dispatch |
| `direct_query_bus::<R>()` | `Arc<dyn QueryBus<R>>` | In-process query dispatch |
| `noop_event_publisher()` | `Arc<dyn EventPublisher>` | Drop events (dev/test) |
| `new_handler_registry()` | `Arc<HandlerRegistry<Req, Resp>>` | Standard handler registry |
| `new_service_registry()` | `Arc<ServiceRegistry<Req, Resp>>` | Standard service registry |

## Dependencies

`edge-domain` has no dependency on ingress, egress, runtime, or any infrastructure
library. It depends only on `thiserror`, `futures`, `async-trait`, and `parking_lot`.
# test
