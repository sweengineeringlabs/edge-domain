# ADR-006: Observability Domain Primitive — domain mandate

**Status:** Implemented
**Date:** 2026-06-18
**Governing ADR:** [ADR-022](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-022-cloud-native-extension-tier.md) — Cloud-Native Extension Tier
**Relates to:** [ADR-001](ADR-001-security-context-propagation.md) — Security Context Propagation, [ADR-005](ADR-005-command-query-bus-stack.md) — CommandBus and QueryBus Middleware Stack
**GitHub Issue:** [#29](https://github.com/sweengineeringlabs/edge-domain/issues/29) — scaffold edge-domain-observer sub-crate

---

## Mandate

Introduce `edge-domain-observer` as a new sub-crate in this workspace — a peer to `edge-domain-handler`, `edge-domain-service`, `edge-domain-security`. Owns the observability port contracts: `HandlerTracer`, `MetricRegistry`, `LogDrain`, and their primitive sub-traits (`Span`, `Counter`, `Histogram`, `Gauge`). No SDK dependencies — pure trait definitions only, same rule as all `edge-domain-*` sub-crates.

`swe-edge-observ-config` is not modified. It remains the infrastructure bootstrap crate for subscriber configuration and initialisation. This workspace owns the domain-layer port contracts only; `edge-observe` (a future opt-in extension repo) owns the SDK-backed implementations.

---

## New crate: `edge-domain-observer`

Location: `domain-observe/` (peer to `domain-clock/`, `domain-security/`)
Crate name: `edge-domain-observer`
`service_type`: `"observe"`

Zero external deps — only `std` and `thiserror`.

---

## What this workspace owns

### Primitive sub-traits

| Trait | Role | Status |
|---|---|---|
| `Span` | Custom child span — `record(key, value)`, `finish(&self)` | **To build** |
| `Counter` | Monotonically increasing counter — `increment(delta: u64)` | **To build** |
| `Histogram` | Distribution of observed values — `record(value: f64)` | **To build** |
| `Gauge` | Current point-in-time measurement — `set(value: f64)` | **To build** |

### Port contracts

| Trait | Role | Status |
|---|---|---|
| `HandlerTracer` | Custom child spans within handler execution; returns `Box<dyn Span>` | **To build** |
| `MetricRegistry` | Domain-level metrics — `counter()`, `histogram()`, `gauge()` | **To build** |
| `LogDrain` | Structured log record emission — `emit(LogRecord)` | **To build** |

### Types

| Type | Role | Status |
|---|---|---|
| `LogRecord` | Structured log entry carried by `LogDrain::emit` | **To build** |
| `NoopSpan` | No-op `Span` — returned by `NoopHandlerTracer` | **To build** |
| `NoopCounter` | No-op `Counter` — returned by `NoopMetricRegistry` | **To build** |
| `NoopHistogram` | No-op `Histogram` — returned by `NoopMetricRegistry` | **To build** |
| `NoopGauge` | No-op `Gauge` — returned by `NoopMetricRegistry` | **To build** |
| `NoopHandlerTracer` | Test double — returns `NoopSpan` | **To build** |
| `NoopMetricRegistry` | Test double — returns noop metric primitives | **To build** |
| `NoopLogDrain` | Test double — discards all log records | **To build** |

All noop types live under `api/observe/types/noop/` (grouped per `shared_prefix_grouping` arch rule). All traits are infallible — no error enum needed; error handling belongs in SDK-backed impls in `edge-observe`.

### SAF factories

| Factory fn | Returns | Const anchor |
|---|---|---|
| `noop_handler_tracer()` | `impl HandlerTracer` | `HANDLER_TRACER_SVC` |
| `noop_metric_registry()` | `impl MetricRegistry` | `METRIC_REGISTRY_SVC` |
| `noop_log_drain()` | `impl LogDrain` | `LOG_DRAIN_SVC` |

---

## Relationship to existing crates

| Concern | Owner |
|---|---|
| Subscriber bootstrap, TOML tracing config | `swe-edge-observ-config` |
| Runtime-level ingress counters, `MetricsProvider` | `swe-observ-metrics` |
| Pipeline span per handler invocation (`"pipeline.stage"`) | `edge-dispatch` |
| Domain port contracts (this ADR) | `edge-domain-observer` |
| OTel subscriber, Prometheus exporter, OTLP exporter | `edge-observe` (future) |

`HandlerTracer` spans are children of the `"pipeline.stage"` span that `edge-dispatch` already creates — not a replacement for it.

`MetricRegistry` complements `swe-observ-metrics` (ingress-level): domain handlers instrument their own operations through `MetricRegistry`; runtime ingress counting remains in `swe-observ-metrics`.

---

## Boundary rules

**B1 — No SDK deps.** `edge-domain-observer` must not depend on Prometheus, OTel, StatsD, or any external SDK. It defines contracts; infrastructure implements them.

**B2 — No dep on `swe-edge-observ-config`.** The domain primitive must not import from the infra bootstrap crate.

**B3 — Handlers hold `Arc<dyn HandlerTracer>` and `Arc<dyn MetricRegistry>` as constructor fields.** These are not passed through `HandlerContext` — they are per-handler infrastructure, injected at construction.

**B4 — Noop impls are always available.** No feature flag gates them. Any handler depending on this crate can use them in tests unconditionally.

---

## Implementation order (layer-gated TDD)

Per `task_workflow_skill.md` and issues #72 → #73 → #74:

1. **API layer** (#72) — define all primitive sub-traits, port contracts, types, error enums; write tests (RED); implement (GREEN); gate: test + audit + clippy
2. **Core/SPI layer** (#73) — noop impl wiring in `core/`; `spi/mod.rs` anchor (`const _: () = ()`); gate: test + audit + clippy
3. **SAF layer** (#74) — factory fns + SAF const anchors + integration tests + example; gate: test + audit 182/182 + clippy + fmt

**SPI note:** leaf crates use `spi/mod.rs` as a structural anchor only (`const _: () = ()`). No `ObserveSpi` trait — that pattern is only in `edge-domain` (the assembler crate).
