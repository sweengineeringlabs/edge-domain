use edge_domain_observer::{LogRecord, ObserveContext, StdObserveFactory};
use std::sync::Arc;

// ── tracer ───────────────────────────────────────────────────────────────────

/// @covers: ObserveContext::tracer
#[test]
fn test_tracer_returns_handler_tracer_happy() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.tracer().start_span("handler_a", "execute").finish();
}

#[test]
fn test_tracer_empty_ids_no_panic_error() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.tracer().start_span("", "").finish();
}

#[test]
fn test_tracer_multiple_spans_edge() {
    let ctx = StdObserveFactory::noop_observe_context();
    for i in 0..5 {
        ctx.tracer().start_span(&format!("h{i}"), "op").finish();
    }
}

// ── drain ────────────────────────────────────────────────────────────────────

/// @covers: ObserveContext::drain
#[test]
fn test_drain_emits_log_record_happy() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.drain()
        .emit(LogRecord::new("INFO", "handler_a", "started"));
}

#[test]
fn test_drain_empty_fields_no_panic_error() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.drain().emit(LogRecord::new("", "", ""));
}

#[test]
fn test_drain_repeated_emit_no_accumulation_edge() {
    let ctx = StdObserveFactory::noop_observe_context();
    for i in 0..10 {
        ctx.drain()
            .emit(LogRecord::new("DEBUG", "h", &format!("msg {i}")));
    }
}

// ── metrics ──────────────────────────────────────────────────────────────────

/// @covers: ObserveContext::metrics
#[test]
fn test_metrics_counter_increments_happy() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.metrics().counter("req.count").increment(1);
}

#[test]
fn test_metrics_empty_name_no_panic_error() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.metrics().counter("").increment(0);
    ctx.metrics().gauge("").set(0.0);
}

#[test]
fn test_metrics_all_instrument_types_edge() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.metrics().counter("c").increment(1);
    ctx.metrics().gauge("g").set(1.0);
    ctx.metrics().histogram("h").record(1.0);
}

// ── object safety ─────────────────────────────────────────────────────────────

#[test]
fn test_observe_context_is_object_safe_error() {
    fn _takes(_: Arc<dyn ObserveContext>) {}
    let ctx: Box<dyn ObserveContext> = StdObserveFactory::noop_observe_context();
    _takes(Arc::from(ctx));
}

// ── multiple instances ────────────────────────────────────────────────────────

/// @covers: NoopObserve::build_noop_observe_context
#[test]
fn test_noop_observe_context_multiple_instances_independent_edge() {
    let a: Box<dyn ObserveContext> = StdObserveFactory::noop_observe_context();
    let b: Box<dyn ObserveContext> = StdObserveFactory::noop_observe_context();
    a.tracer().start_span("a", "op").finish();
    b.tracer().start_span("b", "op").finish();
}
