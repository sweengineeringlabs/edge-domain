# ADR-002: Event Sourcing Pipeline — domain contracts

**Status:** Accepted  
**Date:** 2026-06-12  
**Governing ADR:** [ADR-018](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-018-event-sourcing-pipeline.md) — Event Sourcing Pipeline

---

## Mandate

Define the event sourcing contracts for the `edge-domain` workspace: `DomainEvent`, `Aggregate`, `EventStore<E>`, `EventBus`, `EventPublisher`, `EventSource`, `EventFactory`, `Projection`, `ProjectionFactory`, `Saga`, `SagaRegistry`, `SagaFactory`.

---

## Sub-crate layout

| Sub-crate | Contracts | Intra-domain deps |
|---|---|---|
| `edge-domain-event` | `DomainEvent`, `Aggregate`, `EventStore<E>`, `EventBus`, `EventPublisher`, `EventSource`, `EventFactory` | none |
| `edge-domain-projection` | `Projection`, `ProjectionFactory` | `edge-domain-event` |
| `edge-domain-saga` | `Saga`, `SagaRegistry<S>`, `SagaFactory` | `edge-domain-event`, `edge-domain-command` |

`edge-domain-event` is a leaf crate — it must not import any other `edge-domain-*` crate.

---

## `edge-domain-event` contracts

### `DomainEvent`

```rust
pub trait DomainEvent: Send + Sync {
    fn event_type(&self) -> &str;        // "order.created"
    fn aggregate_id(&self) -> &str;
    fn occurred_at(&self) -> SystemTime;
}
```

### `Aggregate`

```rust
pub trait Aggregate: Default + Send + Sync + 'static {
    type Event: DomainEvent + Send + Sync + Clone + 'static;

    fn apply(&mut self, event: &Self::Event);
    fn id(&self) -> &str;
}
```

Reconstruction: call `apply` for each `EventEnvelope<E>.event` returned by `EventStore::load` in sequence order.

### `EventStore<E>`

```rust
pub trait EventStore<E>: Send + Sync
where E: DomainEvent + Send + 'static
{
    fn append(&self, aggregate_id: &str, events: Vec<E>, expected: ExpectedVersion)
        -> BoxFuture<'_, Result<u64, EventStoreError>>;

    fn load(&self, aggregate_id: &str)
        -> BoxFuture<'_, Result<Vec<EventEnvelope<E>>, EventStoreError>>;

    fn load_from(&self, aggregate_id: &str, from_sequence: u64)
        -> BoxFuture<'_, Result<Vec<EventEnvelope<E>>, EventStoreError>>;
}
```

`ExpectedVersion::Exact(n)` enforces optimistic concurrency. `ExpectedVersion::Any` is for tests and migrations only.

### `EventBus` / `EventPublisher`

```rust
pub trait EventBus: Send + Sync {
    fn publish(&self, event: Arc<dyn DomainEvent>) -> BoxFuture<'_, Result<(), EventError>>;
    fn subscribe(&self) -> EventReceiver;
}

pub trait EventPublisher: Send + Sync {
    fn publish<'a>(&'a self, event: &'a dyn DomainEvent)
        -> BoxFuture<'a, Result<(), EventError>>;
}
```

Use `EventBus` when the caller also needs to consume events. Use `EventPublisher` for emit-only paths (e.g., handler post-write).

### `EventFactory` (SAF)

```rust
pub trait EventFactory {
    fn in_process_bus(config: EventBusConfig) -> InProcessEventBus;
    fn noop_bus()                              -> NoopEventBus;
    fn noop_publisher()                        -> NoopEventPublisher;
    fn in_memory_store<E>()                    -> InMemoryEventStore<E>;
    fn closed_source()                         -> ClosedEventSource;
}
```

Callers: `struct Events; impl EventFactory for Events {}` then call `Events::in_memory_store::<MyEvent>()`.

---

## `edge-domain-projection` contracts

```rust
pub trait Projection: Send + Sync {
    type Event: DomainEvent;
    type ReadModel;

    fn apply(&mut self, event: &Self::Event);
    fn read_model(&self) -> &Self::ReadModel;
}

pub trait ProjectionFactory {
    fn in_memory<E, R, F>(initial: R, reducer: F) -> InMemoryProjection<E, R, F>;
    fn try_drain<E, R, F>(
        projection: &mut InMemoryProjection<E, R, F>,
        events: &[E],
    ) -> Result<usize, ProjectionError>;
}
```

`apply` is infallible. A projection that encounters a malformed event records the anomaly in `ReadModel`; it does not abort fan-out.

---

## `edge-domain-saga` contracts

```rust
pub trait Saga: Send + Sync {
    type SagaId: Eq + Hash + Clone + Send + Sync;
    type Event: DomainEvent;
    type Command: Command;

    fn handle(&mut self, event: &Self::Event) -> Vec<Self::Command>;
    fn is_complete(&self) -> bool;
}

pub trait SagaRegistry<S: Saga>: Send + Sync {
    fn register(&mut self, id: S::SagaId, saga: S) -> Result<(), SagaError>;
    fn get(&self, id: &S::SagaId) -> Result<&S, SagaError>;
}

pub trait SagaFactory {
    fn in_memory_registry<S: Saga>() -> InMemorySagaRegistry<S>;
}
```

The caller dispatches the `Vec<Command>` returned by `handle`; sagas do not call `CommandBus` directly.

---

## SEA module layout (per sub-crate)

```
src/
├── api/          # Traits and types — all public
│   ├── <theme>/traits/   ← DomainEvent, EventStore, Projection, Saga …
│   └── <theme>/types/    ← EventEnvelope, ExpectedVersion, InMemory* …
├── core/         # Implementations — pub(crate) only
├── saf/          # Re-export surface — EventFactory, ProjectionFactory, SagaFactory
└── lib.rs
```

---

## Testing

- All unit tests use `EventFactory::in_memory_store()` — no external deps.
- Projections: `ProjectionFactory::try_drain(projection, &events)` then assert `projection.read_model()`.
- Sagas: call `saga.handle(&event)` with a synthetic event, assert returned commands.
- `#[tokio::test]` for all async tests.
- Naming: `test_<action>_<condition>_<expectation>_happy/error/edge`.
