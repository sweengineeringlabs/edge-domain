use edge_domain_observer::{LogRecord, ObserveBootstrap, StdObserveFactory};

// --- std_factory ---

#[test]
fn test_std_factory_returns_usable_factory_happy() {
    let factory = StdObserveFactory::std_factory();
    let tracer = factory.build_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
}

#[test]
fn test_std_factory_called_multiple_times_error() {
    let _a = StdObserveFactory::std_factory();
    let _b = StdObserveFactory::std_factory();
}

#[test]
fn test_std_factory_all_three_primitives_edge() {
    let factory = StdObserveFactory::std_factory();
    let _ = factory.build_handler_tracer();
    let _ = factory.build_metric_registry();
    let _ = factory.build_log_drain();
}

// --- validate ---

#[test]
fn test_validate_initialized_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let result = factory.validate();
    assert_eq!(result, Ok(()), "factory.validate() should return Ok");
}

#[test]
fn test_validate_std_factory_returns_ok_error() {
    let factory = StdObserveFactory;
    let result = factory.validate();
    assert_eq!(result, Ok(()), "StdObserveFactory.validate() should return Ok");
}

#[test]
fn test_validate_called_multiple_times_edge() {
    let factory = StdObserveFactory::create_factory();
    let first = factory.validate();
    let second = factory.validate();
    assert_eq!(first, Ok(()), "first validate should succeed");
    assert_eq!(second, Ok(()), "second validate should also succeed");
}

// --- build_handler_tracer ---

#[test]
fn test_build_handler_tracer_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let tracer = factory.build_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
}

#[test]
fn test_build_handler_tracer_uninitialized_error() {
    let factory = StdObserveFactory;
    let tracer = factory.build_handler_tracer();
    let span = tracer.start_span("", "");
    span.finish();
}

#[test]
fn test_build_handler_tracer_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let t1 = factory.build_handler_tracer();
    let t2 = factory.build_handler_tracer();
    t1.start_span("a", "op").finish();
    t2.start_span("b", "op").finish();
}

// --- build_metric_registry ---

#[test]
fn test_build_metric_registry_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let registry = factory.build_metric_registry();
    registry.counter("c").increment(1);
}

#[test]
fn test_build_metric_registry_uninitialized_error() {
    let factory = StdObserveFactory;
    let registry = factory.build_metric_registry();
    registry.gauge("g").set(0.0);
}

#[test]
fn test_build_metric_registry_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let r1 = factory.build_metric_registry();
    let r2 = factory.build_metric_registry();
    r1.counter("a").increment(1);
    r2.counter("b").increment(2);
}

// --- build_log_drain ---

#[test]
fn test_build_log_drain_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let drain = factory.build_log_drain();
    drain.emit(LogRecord::new("INFO", "h", "msg"));
}

#[test]
fn test_build_log_drain_uninitialized_error() {
    let factory = StdObserveFactory;
    let drain = factory.build_log_drain();
    drain.emit(LogRecord::new("ERROR", "h", "err"));
}

#[test]
fn test_build_log_drain_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let d1 = factory.build_log_drain();
    let d2 = factory.build_log_drain();
    d1.emit(LogRecord::new("INFO", "a", "1"));
    d2.emit(LogRecord::new("WARN", "b", "2"));
}

#[test]
fn test_observe_factory_all_three_primitives_independently_usable() {
    let factory = StdObserveFactory::create_factory();
    let tracer = factory.build_handler_tracer();
    let registry = factory.build_metric_registry();
    let drain = factory.build_log_drain();

    let span = tracer.start_span("pipeline_handler", "execute");
    span.record("db.rows", "42");
    span.finish();

    registry.counter("pipeline.requests").increment(1);
    registry.histogram("pipeline.duration_ms").record(25.0);

    drain.emit(LogRecord::new("INFO", "pipeline_handler", "done"));
}

#[test]
fn test_observe_factory_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let factory = StdObserveFactory::create_factory();
    assert_send_sync(&factory);
}
