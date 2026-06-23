use edge_domain_observer::{LogRecord, ObserveBootstrap, StdObserveFactory, OBSERVE_FACTORY_SVC};

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_validate_passes_happy() {
    let factory = StdObserveFactory::create_factory();
    let result = factory.validate();
    assert_eq!(result, Ok(()), "created factory should validate successfully");
}

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_builds_noop_triplet_error() {
    let factory = StdObserveFactory::create_factory();
    let span = factory
        .build_handler_tracer()
        .start_span("h", "op");
    span.finish();
    factory
        .build_log_drain()
        .emit(LogRecord::new("WARN", "h", "bad input"));
    factory.build_metric_registry().counter("errs").increment(1);
    assert_eq!(std::mem::size_of_val(&*span), 0, "factory builds ZST triplet");
}

// @covers StdObserveFactory::create_factory
#[test]
fn test_create_factory_svc_called_repeatedly_edge() {
    let mut last_factory = StdObserveFactory::create_factory();
    for _ in 0..5 {
        last_factory = StdObserveFactory::create_factory();
    }
    let tracer = last_factory.build_handler_tracer();
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "factories work when called repeatedly");
}

#[test]
fn test_observe_factory_svc_key_namespaced_happy() {
    assert!(OBSERVE_FACTORY_SVC.starts_with("edge."));
}
