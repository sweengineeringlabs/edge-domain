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
| `DomainEvent` | Immutable fact that something happened |
| `EventPublisher` | Emits domain events to subscribers |
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
    EventPublisher, in_memory_repository, noop_event_publisher, new_handler_registry,
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
    // Development defaults (swap with real infrastructure in production)
    let repo      = in_memory_repository::<String, String>();
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
    let resp = self.service.execute(req).await?;  // ServiceError   → HandlerError
    self.bus.dispatch(Box::new(cmd)).await?;       // CommandError   → HandlerError
    Ok(resp)
}
```

## Default implementations

Provided for development and testing — swap with real infrastructure in production:

| Factory | Returns | Use case |
|---------|---------|---------|
| `in_memory_repository::<T, Id>()` | `Arc<dyn Repository<T, Id>>` | Tests, local dev |
| `direct_command_bus()` | `Arc<dyn CommandBus>` | In-process command dispatch |
| `direct_query_bus::<R>()` | `Arc<dyn QueryBus<R>>` | In-process query dispatch |
| `noop_event_publisher()` | `Arc<dyn EventPublisher>` | Drop events (dev/test) |
| `new_handler_registry()` | `Arc<HandlerRegistry<Req, Resp>>` | Standard handler registry |
| `new_service_registry()` | `Arc<ServiceRegistry<Req, Resp>>` | Standard service registry |

## Dependencies

`edge-domain` has no dependency on ingress, egress, runtime, or any infrastructure
library. It depends only on `thiserror`, `async-trait`, and `parking_lot`.
