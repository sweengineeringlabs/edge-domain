use edge_domain_observer::{
    Counter, Gauge, HandlerTracer, Histogram, LogDrain, MetricRegistry, NoopObserve,
    ObserverContext, Span, StdObserveFactory,
};

// --- build_noop_counter ---

#[test]
fn test_build_noop_counter_returns_usable_counter_happy() {
    let c = StdObserveFactory::build_noop_counter();
    c.increment(1);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}

#[test]
fn test_build_noop_counter_max_delta_no_panic_error() {
    let c = StdObserveFactory::build_noop_counter();
    c.increment(u64::MAX);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}

#[test]
fn test_build_noop_counter_multiple_instances_independent_edge() {
    let a = StdObserveFactory::build_noop_counter();
    let b = StdObserveFactory::build_noop_counter();
    a.increment(1);
    b.increment(2);
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop counter is ZST");
}

// --- build_noop_gauge ---

#[test]
fn test_build_noop_gauge_returns_usable_gauge_happy() {
    let g = StdObserveFactory::build_noop_gauge();
    g.set(42.0);
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_build_noop_gauge_negative_value_no_panic_error() {
    let g = StdObserveFactory::build_noop_gauge();
    g.set(-1.0);
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_build_noop_gauge_multiple_instances_independent_edge() {
    let a = StdObserveFactory::build_noop_gauge();
    let b = StdObserveFactory::build_noop_gauge();
    a.set(0.0);
    b.set(f64::MAX);
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop gauge is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop gauge is ZST");
}

// --- build_noop_histogram ---

#[test]
fn test_build_noop_histogram_returns_usable_histogram_happy() {
    let h = StdObserveFactory::build_noop_histogram();
    h.record(25.0);
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
}

#[test]
fn test_build_noop_histogram_zero_value_no_panic_error() {
    let h = StdObserveFactory::build_noop_histogram();
    h.record(0.0);
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
}

#[test]
fn test_build_noop_histogram_multiple_instances_independent_edge() {
    let a = StdObserveFactory::build_noop_histogram();
    let b = StdObserveFactory::build_noop_histogram();
    a.record(1.0);
    b.record(2.0);
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop histogram is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop histogram is ZST");
}

// --- build_noop_span ---

#[test]
fn test_build_noop_span_returns_usable_span_happy() {
    let s = StdObserveFactory::build_noop_span();
    s.record("k", "v");
    s.finish();
    assert_eq!(std::mem::size_of_val(&*s), 0, "noop span is ZST");
}

#[test]
fn test_build_noop_span_empty_key_value_no_panic_error() {
    let s = StdObserveFactory::build_noop_span();
    s.record("", "");
    s.finish();
    assert_eq!(std::mem::size_of_val(&*s), 0, "noop span is ZST");
}

#[test]
fn test_build_noop_span_multiple_finish_calls_edge() {
    let a = StdObserveFactory::build_noop_span();
    let b = StdObserveFactory::build_noop_span();
    a.finish();
    b.finish();
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop span is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop span is ZST");
}

// --- build_noop_handler_tracer ---

#[test]
fn test_build_noop_handler_tracer_returns_usable_tracer_happy() {
    let t = StdObserveFactory::build_noop_handler_tracer();
    t.start_span("h", "op").finish();
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_build_noop_handler_tracer_empty_ids_no_panic_error() {
    let t = StdObserveFactory::build_noop_handler_tracer();
    t.start_span("", "").finish();
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

#[test]
fn test_build_noop_handler_tracer_multiple_spans_edge() {
    let t = StdObserveFactory::build_noop_handler_tracer();
    for i in 0..5 {
        t.start_span(&format!("h{i}"), "op").finish();
    }
    assert_eq!(std::mem::size_of_val(&*t), 0, "noop handler tracer is ZST");
}

// --- build_noop_log_drain ---

#[test]
fn test_build_noop_log_drain_returns_usable_drain_happy() {
    use edge_domain_observer::LogRecord;
    let d = StdObserveFactory::build_noop_log_drain();
    d.emit(LogRecord::new("INFO", "h", "msg"));
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

#[test]
fn test_build_noop_log_drain_empty_fields_no_panic_error() {
    use edge_domain_observer::LogRecord;
    let d = StdObserveFactory::build_noop_log_drain();
    d.emit(LogRecord::new("", "", ""));
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

#[test]
fn test_build_noop_log_drain_multiple_emits_no_accumulation_edge() {
    use edge_domain_observer::LogRecord;
    let d = StdObserveFactory::build_noop_log_drain();
    for i in 0..10 {
        d.emit(LogRecord::new("DEBUG", "h", &format!("{i}")));
    }
    assert_eq!(std::mem::size_of_val(&*d), 0, "noop log drain is ZST");
}

// --- build_noop_metric_registry ---

#[test]
fn test_build_noop_metric_registry_returns_usable_registry_happy() {
    let r = StdObserveFactory::build_noop_metric_registry();
    r.counter("req").increment(1);
    assert_eq!(std::mem::size_of_val(&*r), 0, "noop metric registry is ZST");
}

#[test]
fn test_build_noop_metric_registry_empty_name_no_panic_error() {
    let r = StdObserveFactory::build_noop_metric_registry();
    r.counter("").increment(0);
    r.gauge("").set(0.0);
    r.histogram("").record(0.0);
    assert_eq!(std::mem::size_of_val(&*r), 0, "noop metric registry is ZST");
}

#[test]
fn test_build_noop_metric_registry_all_instruments_edge() {
    let r = StdObserveFactory::build_noop_metric_registry();
    r.counter("c").increment(1);
    r.gauge("g").set(1.0);
    r.histogram("h").record(1.0);
    assert_eq!(std::mem::size_of_val(&*r), 0, "noop metric registry is ZST");
}

// --- trait object safety / return types ---

#[test]
fn test_noop_observe_methods_return_correct_dyn_types_happy() {
    fn _takes_counter(_: Box<dyn Counter>) {}
    fn _takes_gauge(_: Box<dyn Gauge>) {}
    fn _takes_histogram(_: Box<dyn Histogram>) {}
    fn _takes_span(_: Box<dyn Span>) {}
    fn _takes_tracer(_: Box<dyn HandlerTracer>) {}
    fn _takes_drain(_: Box<dyn LogDrain>) {}
    fn _takes_registry(_: Box<dyn MetricRegistry>) {}

    let c = StdObserveFactory::build_noop_counter();
    let g = StdObserveFactory::build_noop_gauge();
    let h = StdObserveFactory::build_noop_histogram();
    let s = StdObserveFactory::build_noop_span();
    let t = StdObserveFactory::build_noop_handler_tracer();
    let d = StdObserveFactory::build_noop_log_drain();
    let r = StdObserveFactory::build_noop_metric_registry();

    _takes_counter(c);
    _takes_gauge(g);
    _takes_histogram(h);
    _takes_span(s);
    _takes_tracer(t);
    _takes_drain(d);
    _takes_registry(r);

    // Trait object conversions work correctly
    assert!(true);
}

// --- noop_name ---

/// @covers: NoopObserve::noop_name
#[test]
fn test_noop_name_returns_nonempty_identifier_happy() {
    let name = StdObserveFactory.noop_name();
    assert!(
        !name.is_empty(),
        "noop_name must return a non-empty identifier"
    );
}

/// @covers: NoopObserve::noop_name
#[test]
fn test_noop_name_default_is_stable_error() {
    // Calling on a second instance must return the same value (no state leakage).
    let first = StdObserveFactory.noop_name();
    let second = StdObserveFactory.noop_name();
    assert_eq!(first, second);
    assert!(!first.is_empty(), "noop_name should not be empty");
}

/// @covers: NoopObserve::noop_name
#[test]
fn test_noop_name_callable_via_trait_object_edge() {
    let f: &dyn NoopObserve = &StdObserveFactory;
    let name = f.noop_name();
    assert!(!name.is_empty(), "noop_name via trait object should not be empty");
}

// --- build_noop_observer_context ---

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_build_noop_observer_context_returns_usable_context_happy() {
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
    ctx.tracer().start_span("h", "op").finish();
    assert_eq!(std::mem::size_of_val(&*ctx), 0, "noop observer context is ZST");
}

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_build_noop_observer_context_empty_ids_no_panic_error() {
    let ctx: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
    ctx.tracer().start_span("", "").finish();
    assert_eq!(std::mem::size_of_val(&*ctx), 0, "noop observer context is ZST");
}

/// @covers: NoopObserve::build_noop_observer_context
#[test]
fn test_build_noop_observer_context_multiple_instances_independent_edge() {
    let a: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
    let b: Box<dyn ObserverContext> = StdObserveFactory::build_noop_observer_context();
    a.metrics().counter("a").increment(1);
    b.metrics().counter("b").increment(2);
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop observer context is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop observer context is ZST");
}
