use edge_domain_observer::{HandlerTracer, StdObserveFactory, HANDLER_TRACER_SVC};

#[test]
fn test_noop_handler_tracer_svc_builds_usable_tracer_happy() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    tracer.start_span("handler_a", "execute").finish();
}

#[test]
fn test_noop_handler_tracer_svc_empty_handler_id_error() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    tracer.start_span("", "").finish();
}

#[test]
fn test_noop_handler_tracer_svc_multiple_spans_edge() {
    let tracer = StdObserveFactory::noop_handler_tracer();
    for i in 0..3 {
        tracer.start_span(&format!("h{i}"), "op").finish();
    }
}

#[test]
fn test_handler_tracer_svc_key_namespaced_happy() {
    assert!(HANDLER_TRACER_SVC.starts_with("edge."));
}

#[test]
fn test_handler_tracer_svc_returns_dyn_trait_object() {
    let _tracer: Box<dyn HandlerTracer> = StdObserveFactory::noop_handler_tracer();
}
