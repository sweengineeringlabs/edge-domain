# ADR-005: CommandBus and QueryBus Middleware Stack — domain mandate

**Status:** Accepted  
**Date:** 2026-06-15  
**Governing ADR:** [ADR-025](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-025-commandbus-middleware-stack.md) — CommandBus and QueryBus Middleware Stack  
**Relates to:** [ADR-024](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-024-handler-execute-contract.md) — Handler execute contract

---

## Mandate

`edge-domain-command` and `edge-domain-query` own the write and read bus contracts. Both already expose a factory trait (`CommandBusFactory`, `QueryBusFactory`) designed for extension. The `EventBus` pattern — `InProcessEventBus` + `NoopEventBus` in `edge-domain-event` — is the precedent all new bus implementations follow.

---

## What this workspace owns

### edge-domain-command

| Type | Role | Status |
|---|---|---|
| `Command` trait | Write operation contract — named, void, self-contained | Exists |
| `CommandBus` trait | Single dispatch point for all write operations | Exists |
| `CommandBusFactory` trait | Extensible factory for bus variants | Exists |
| `DirectCommandBus` | Inline dispatch, same task | Exists |
| `NoopCommand` | Structural placeholder — always `Ok(())` | Exists |
| `StdCommandBusFactory` | Standard factory marker | Exists |
| `NoopCommandBus` | Discards all commands silently | **To build** |
| `LoggingCommandBus` | Wraps inner bus; traces `cmd.name()` + outcome | **To build** |

### edge-domain-query

| Type | Role | Status |
|---|---|---|
| `Query` trait | Read operation contract — named, typed result, no side effects | Exists |
| `QueryBus<R>` trait | Single dispatch point for all read operations | Exists |
| `QueryBusFactory` trait | Extensible factory for bus variants | Exists |
| `DirectQueryBus<R>` | Inline dispatch, same task | Exists |
| `NoopQuery` | Structural placeholder — always `Ok(())` | Exists |
| `StdQueryBusFactory` | Standard factory marker | Exists |
| `NoopQueryBus<R>` | Returns `QueryError::NotFound` for all queries | **To build** |
| `LoggingQueryBus<R>` | Wraps inner bus; traces `query.name()` + outcome | **To build** |

---

## Extension pattern

New bus implementations follow the decorator pattern. They wrap an inner bus and delegate dispatch:

```rust
pub struct LoggingCommandBus {
    inner: Arc<dyn CommandBus>,
}

impl CommandBus for LoggingCommandBus {
    fn dispatch(&self, cmd: Box<dyn Command>) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move {
            let name = cmd.name().to_string();
            tracing::info!(command = %name, "dispatching");
            let result = self.inner.dispatch(cmd).await;
            match &result {
                Ok(_) => tracing::info!(command = %name, "completed"),
                Err(e) => tracing::error!(command = %name, error = %e, "failed"),
            }
            result
        })
    }
}
```

Each new implementation adds a factory method to `CommandBusFactory` / `QueryBusFactory`. Callers always receive `Arc<dyn CommandBus>` — never a concrete type.

---

## Boundary rules

**B1 — No infrastructure deps in this workspace.** `edge-domain-command` and `edge-domain-query` must not depend on brokers, databases, or external transports. Bus implementations that require infrastructure belong in the crate that owns that infrastructure.

**B2 — CommandBus is the only write entry point.** `Handler::execute()` dispatches writes exclusively through `ctx.commands` (ADR-024 S9). Direct repository mutation from a handler body is prohibited.

**B3 — QueryBus injected at construction.** Read handlers hold `Arc<dyn QueryBus<Result=R>>` as a field. They do not receive it via `HandlerContext` (ADR-024 S10).

**B4 — Factory methods return concrete types, callers erase to trait.** Factory methods on `CommandBusFactory` return the concrete implementation type. Call sites immediately wrap in `Arc<dyn CommandBus>`.
