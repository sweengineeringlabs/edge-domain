use edge_domain_observe::{LogRecord, ObserveContext, StdObserveFactory, OBSERVE_CONTEXT_SVC};

// ── factory method ────────────────────────────────────────────────────────────

#[test]
fn test_noop_observe_context_svc_builds_usable_context_happy() {
    let ctx: Box<dyn ObserveContext> = StdObserveFactory::noop_observe_context();
    ctx.tracer().start_span("h", "op").finish();
    ctx.drain().emit(LogRecord::new("INFO", "h", "msg"));
    ctx.metrics().counter("c").increment(1);
}

#[test]
fn test_noop_observe_context_svc_tracer_no_panic_happy() {
    let ctx = StdObserveFactory::noop_observe_context();
    for i in 0..3 {
        ctx.tracer().start_span(&format!("h{i}"), "op").finish();
    }
}

// ── service key ───────────────────────────────────────────────────────────────

#[test]
fn test_observe_context_svc_key_namespaced_happy() {
    assert!(OBSERVE_CONTEXT_SVC.starts_with("edge."));
}

#[test]
fn test_noop_observe_context_svc_empty_span_ids_no_panic_error() {
    let ctx = StdObserveFactory::noop_observe_context();
    ctx.tracer().start_span("", "").finish();
    ctx.drain().emit(LogRecord::new("", "", ""));
}

// ── independence ──────────────────────────────────────────────────────────────

#[test]
fn test_noop_observe_context_svc_multiple_calls_independent_edge() {
    let a = StdObserveFactory::noop_observe_context();
    let b = StdObserveFactory::noop_observe_context();
    a.metrics().gauge("g").set(1.0);
    b.metrics().gauge("g").set(2.0);
    // Both instances operate without interference.
}
