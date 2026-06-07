# edge-domain

## WHAT

The L2 domain contract for swe-edge. Defines the execution units and port
contracts consumed by ingress, egress, and the runtime — no knowledge of
transport protocols, databases, or messaging infrastructure.

Key capabilities:
- **`Handler<Req, Resp>`** — ingress-facing execution unit; the primary port contract
- **`Service<Req, Resp>`** — domain operation called by handlers or background jobs
- **`Repository<T, Id>` / `QueryableRepository<T, Id>`** — data access contracts
- **`EventBus` / `EventPublisher` / `EventStore<E>`** — provider-agnostic event sourcing
- **`Command` / `Query` CQRS buses** — in-process command and query dispatch
- **Error bridging** — all domain errors convert to `HandlerError` via `?`

## WHY

| Problem | Solution |
|---------|----------|
| Infrastructure details leak into business logic | Domain owns only traits and errors; no dep on ingress, egress, or runtime |
| Handler, service, and repository impls diverge across teams | Single port contract — all consumers implement the same `Handler<Req, Resp>` |
| Diamond dep conflicts when domain types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
| Boilerplate error conversion at every adapter boundary | `RepositoryError`, `ServiceError`, `CommandError`, `QueryError` all `impl From<_> for HandlerError` |
| Provider-specific event bus tied to domain logic | `EventBus` and `EventStore` are traits; Tokio backend lives in `spi/`, swappable without touching domain code |
