use edge_domain_observe::{HandlerTracer, StdObserveFactory};

// --- start_span (HandlerTracer) ---

#[test]
fn test_start_span_handler_and_op_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("handler_b", "validate");
    span.record("result", "ok");
    span.finish();
}

#[test]
fn test_start_span_empty_handler_id_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("", "op");
    span.finish();
}

#[test]
fn test_start_span_very_long_ids_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    let long_id = "x".repeat(1024);
    let span = tracer.start_span(&long_id, &long_id);
    span.finish();
}

#[test]
fn test_handler_tracer_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let tracer = StdObserveFactory::noop_handler_tracer();
    assert_send_sync(&tracer);
}

#[test]
fn test_handler_tracer_returns_dyn_trait_object() {
    let tracer: Box<dyn HandlerTracer> = StdObserveFactory::noop_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
}
