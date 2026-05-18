# Changelog

## Unreleased

### Added
- `Aggregate` trait — state rebuilt by replaying domain events; `apply(&mut self, event)` + `id()`.
- `EventStore<E>` trait — provider-agnostic append-only event stream: `append`, `load`, `load_from`.
- `EventEnvelope<E>` — domain event with store metadata (sequence, aggregate_id, occurred_at).
- `EventStoreError` — typed errors: `Conflict { aggregate_id, expected, actual }`, `Unavailable`, `Internal`.
- `ExpectedVersion` — optimistic concurrency: `Any`, `NoStream`, `Exact(u64)`.
- `new_in_memory_event_store::<E>()` — SAF factory returning `Arc<dyn EventStore<E>>` backed by `parking_lot::RwLock<HashMap>`.
- `reconstitute::<A>(store, id)` — SAF helper: loads all events and replays them into an `Aggregate`.
- `OutboundRegistry<H>` — thread-safe egress handle registry mirroring `HandlerRegistry`.
- `api/traits.rs` — SEA interface contract (`Gateway`, `Validator`) for the domain layer.
- `src/gateway/` — domain public entry point layer (SEA rule 131).

### Changed
- `InMemoryRepository` lock changed from `std::sync::RwLock` to `parking_lot::RwLock` — eliminates lock poisoning panic risk.
- `InMemoryEventStore` uses `parking_lot::RwLock` (never poisons).
- `futures` added as a direct dependency (required for `BoxFuture` in `EventStore` trait methods).
- `rust-version` bumped from `1.75` to `1.95`.
