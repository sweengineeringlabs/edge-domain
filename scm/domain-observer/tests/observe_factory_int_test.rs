use edge_domain_observer::{LogRecord, ObserveBootstrap, StdObserveFactory};

// --- std_factory ---

#[test]
fn test_std_factory_returns_usable_factory_happy() {
    let factory = StdObserveFactory::std_factory();
    let tracer = factory.build_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "factory produces ZST spans");
}

#[test]
fn test_std_factory_called_multiple_times_error() {
    let a = StdObserveFactory::std_factory();
    let b = StdObserveFactory::std_factory();
    let tracer = a.build_handler_tracer();
    let span = tracer.start_span("h", "op");
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "factories produce same ZST");
}

#[test]
fn test_std_factory_all_three_primitives_edge() {
    let factory = StdObserveFactory::std_factory();
    let tracer = factory.build_handler_tracer();
    let registry = factory.build_metric_registry();
    let drain = factory.build_log_drain();
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "all primitives are ZST");
    assert_eq!(std::mem::size_of_val(&*registry), 0, "registry is ZST");
    assert_eq!(std::mem::size_of_val(&*drain), 0, "drain is ZST");
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
    assert_eq!(std::mem::size_of_val(&*span), 0, "tracer produces ZST spans");
}

#[test]
fn test_build_handler_tracer_uninitialized_error() {
    let factory = StdObserveFactory;
    let tracer = factory.build_handler_tracer();
    let span = tracer.start_span("", "");
    span.finish();
    assert_eq!(std::mem::size_of_val(&*span), 0, "uninitialized factory produces ZST spans");
}

#[test]
fn test_build_handler_tracer_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let t1 = factory.build_handler_tracer();
    let t2 = factory.build_handler_tracer();
    let s1 = t1.start_span("a", "op");
    let s2 = t2.start_span("b", "op");
    s1.finish();
    s2.finish();
    assert_eq!(std::mem::size_of_val(&*s1), 0, "multiple tracers produce ZST spans");
}

// --- build_metric_registry ---

#[test]
fn test_build_metric_registry_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let registry = factory.build_metric_registry();
    let counter = registry.counter("c");
    counter.increment(1);
    assert_eq!(std::mem::size_of_val(&*counter), 0, "registry produces ZST counters");
}

#[test]
fn test_build_metric_registry_uninitialized_error() {
    let factory = StdObserveFactory;
    let registry = factory.build_metric_registry();
    let gauge = registry.gauge("g");
    gauge.set(0.0);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "uninitialized factory produces ZST gauges");
}

#[test]
fn test_build_metric_registry_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let r1 = factory.build_metric_registry();
    let r2 = factory.build_metric_registry();
    let c1 = r1.counter("a");
    let c2 = r2.counter("b");
    c1.increment(1);
    c2.increment(2);
    assert_eq!(std::mem::size_of_val(&*c1), 0, "multiple registries produce ZST instruments");
}

// --- build_log_drain ---

#[test]
fn test_build_log_drain_std_factory_happy() {
    let factory = StdObserveFactory::create_factory();
    let drain = factory.build_log_drain();
    drain.emit(LogRecord::new("INFO", "h", "msg"));
    assert_eq!(std::mem::size_of_val(&*drain), 0, "factory produces ZST drain");
}

#[test]
fn test_build_log_drain_uninitialized_error() {
    let factory = StdObserveFactory;
    let drain = factory.build_log_drain();
    drain.emit(LogRecord::new("ERROR", "h", "err"));
    assert_eq!(std::mem::size_of_val(&*drain), 0, "uninitialized factory produces ZST drain");
}

#[test]
fn test_build_log_drain_multiple_calls_edge() {
    let factory = StdObserveFactory::create_factory();
    let d1 = factory.build_log_drain();
    let d2 = factory.build_log_drain();
    d1.emit(LogRecord::new("INFO", "a", "1"));
    d2.emit(LogRecord::new("WARN", "b", "2"));
    assert_eq!(std::mem::size_of_val(&*d1), 0, "multiple drains are ZST");
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
    assert_eq!(std::mem::size_of_val(&*span), 0, "all primitives work independently");
}

#[test]
fn test_observe_factory_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let factory = StdObserveFactory::create_factory();
    assert_send_sync(&factory);
    let tracer = factory.build_handler_tracer();
    assert_eq!(std::mem::size_of_val(&*tracer), 0, "factory is Send+Sync");
}
