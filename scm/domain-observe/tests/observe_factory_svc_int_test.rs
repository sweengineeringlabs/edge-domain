use edge_domain_observe::{LogRecord, ObserveFactory, StdObserveFactory, OBSERVE_FACTORY_SVC};

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_validate_passes_happy() {
    let factory = StdObserveFactory::create_factory();
    assert!(factory.validate().is_ok());
}

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_builds_noop_triplet_error() {
    let factory = StdObserveFactory::create_factory();
    factory
        .build_handler_tracer()
        .start_span("h", "op")
        .finish();
    factory
        .build_log_drain()
        .emit(LogRecord::new("WARN", "h", "bad input"));
    factory.build_metric_registry().counter("errs").increment(1);
}

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_called_repeatedly_edge() {
    for _ in 0..5 {
        let _ = StdObserveFactory::create_factory();
    }
}

#[test]
fn test_observe_factory_svc_key_namespaced_happy() {
    assert!(OBSERVE_FACTORY_SVC.starts_with("edge."));
}
