use edge_domain_observer::{LogRecord, ObserverContext, StdObserveFactory};
use std::sync::Arc;

// ── tracer ───────────────────────────────────────────────────────────────────

/// @covers: ObserverContext::tracer
#[test]
fn test_tracer_returns_handler_tracer_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let t = ctx.tracer();
    t.start_span("handler_a", "execute").finish();
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_tracer_empty_ids_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let t = ctx.tracer();
    t.start_span("", "").finish();
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_tracer_multiple_spans_edge() {
    let ctx = StdObserveFactory::noop_observer_context();
    let t = ctx.tracer();
    for i in 0..5 {
        t.start_span(&format!("h{i}"), "op").finish();
    }
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

// ── drain ────────────────────────────────────────────────────────────────────

/// @covers: ObserverContext::drain
#[test]
fn test_drain_emits_log_record_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let d = ctx.drain();
    d.emit(LogRecord::new("INFO", "handler_a", "started"));
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

#[test]
fn test_drain_empty_fields_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let d = ctx.drain();
    d.emit(LogRecord::new("", "", ""));
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

#[test]
fn test_drain_repeated_emit_no_accumulation_edge() {
    let ctx = StdObserveFactory::noop_observer_context();
    let d = ctx.drain();
    for i in 0..10 {
        d.emit(LogRecord::new("DEBUG", "h", &format!("msg {i}")));
    }
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

// ── metrics ──────────────────────────────────────────────────────────────────

/// @covers: ObserverContext::metrics
#[test]
fn test_metrics_counter_increments_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let r = ctx.metrics();
    let c = r.counter("req.count");
    c.increment(1);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}

#[test]
fn test_metrics_empty_name_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let r = ctx.metrics();
    let c = r.counter("");
    let g = r.gauge("");
    c.increment(0);
    g.set(0.0);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_metrics_all_instrument_types_edge() {
    let ctx = StdObserveFactory::noop_observer_context();
    let r = ctx.metrics();
    let c = r.counter("c");
    let g = r.gauge("g");
    let h = r.histogram("h");
    c.increment(1);
    g.set(1.0);
    h.record(1.0);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
}

// ── object safety ─────────────────────────────────────────────────────────────

#[test]
fn test_observe_context_is_object_safe_error() {
    fn _takes(_: Arc<dyn ObserverContext>) {}
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    let a: Arc<dyn ObserverContext> = Arc::from(ctx);
    _takes(a.clone());
    assert!(Arc::strong_count(&a) > 0, "Arc should be valid after passing to function");
}

// ── multiple instances ────────────────────────────────────────────────────────

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_noop_observer_context_multiple_instances_independent_edge() {
    let a: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    let b: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    let ta = a.tracer();
    let tb = b.tracer();
    ta.start_span("a", "op").finish();
    tb.start_span("b", "op").finish();
    assert_eq!(std::mem::size_of_val(&*ta), 0, "noop handler tracer is ZST");
    assert_eq!(std::mem::size_of_val(&*tb), 0, "noop handler tracer is ZST");
}
