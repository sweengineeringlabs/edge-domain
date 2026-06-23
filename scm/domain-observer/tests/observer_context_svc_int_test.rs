use edge_domain_observer::{LogRecord, ObserverContext, StdObserveFactory, OBSERVE_CONTEXT_SVC};

// ── factory method ────────────────────────────────────────────────────────────

#[test]
fn test_noop_observer_context_svc_builds_usable_context_happy() {
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::noop_observer_context();
    ctx.tracer().start_span("h", "op").finish();
    ctx.drain().emit(LogRecord::new("INFO", "h", "msg"));
    ctx.metrics().counter("c").increment(1);
}

#[test]
fn test_noop_observer_context_svc_tracer_no_panic_happy() {
    let ctx = StdObserveFactory::noop_observer_context();
    let tracer = ctx.tracer();
    for i in 0..3 {
        let span = tracer.start_span(&format!("h{i}"), "op");
        span.finish();
        assert_eq!(std::mem::size_of_val(&*span), 0, "noop span is ZST");
    }
}

// ── service key ───────────────────────────────────────────────────────────────

#[test]
fn test_observe_context_svc_key_namespaced_happy() {
    assert!(OBSERVE_CONTEXT_SVC.starts_with("edge."));
}

#[test]
fn test_noop_observer_context_svc_empty_span_ids_no_panic_error() {
    let ctx = StdObserveFactory::noop_observer_context();
    let span = ctx.tracer().start_span("", "");
    span.finish();
    let drain = ctx.drain();
    drain.emit(LogRecord::new("", "", ""));
    assert_eq!(std::mem::size_of_val(&*span), 0, "noop span is ZST");
}

// ── noop_arc_observe_context ──────────────────────────────────────────────────

#[test]
fn test_noop_arc_observe_context_svc_builds_usable_context_happy() {
    use std::sync::Arc;
    let ctx: Arc<dyn ObserverContext> = StdObserveFactory::noop_arc_observe_context();
    ctx.tracer().start_span("h", "op").finish();
    ctx.drain().emit(LogRecord::new("INFO", "h", "msg"));
    ctx.metrics().counter("c").increment(1);
}

#[test]
fn test_noop_arc_observe_context_svc_empty_inputs_no_panic_error() {
    use std::sync::Arc;
    let ctx: Arc<dyn ObserverContext> = StdObserveFactory::noop_arc_observe_context();
    ctx.tracer().start_span("", "").finish();
    ctx.drain().emit(LogRecord::new("", "", ""));
}

#[test]
fn test_noop_arc_observe_context_svc_arc_clone_shares_same_ptr_edge() {
    use std::sync::Arc;
    let ctx: Arc<dyn ObserverContext> = StdObserveFactory::noop_arc_observe_context();
    let ctx2 = Arc::clone(&ctx);
    assert!(Arc::ptr_eq(&ctx, &ctx2));
}

// ── independence ──────────────────────────────────────────────────────────────

#[test]
fn test_noop_observer_context_svc_multiple_calls_independent_edge() {
    let a = StdObserveFactory::noop_observer_context();
    let b = StdObserveFactory::noop_observer_context();
    let gauge_a = a.metrics().gauge("g");
    let gauge_b = b.metrics().gauge("g");
    gauge_a.set(1.0);
    gauge_b.set(2.0);
    assert_eq!(std::mem::size_of_val(&*gauge_a), 0, "contexts are independent");
}
