use edge_domain_observer::{MetricRegistry, StdObserveFactory};

// --- counter (MetricRegistry) ---

#[test]
fn test_counter_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("requests.total");
    counter.increment(1);
    assert_eq!(std::mem::size_of_val(&*counter), 0, "noop counter is ZST");
}

#[test]
fn test_counter_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("");
    counter.increment(1);
    assert_eq!(std::mem::size_of_val(&*counter), 0, "noop counter is ZST");
}

#[test]
fn test_counter_multiple_instruments_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let a = registry.counter("a");
    let b = registry.counter("b");
    a.increment(1);
    b.increment(2);
    a.increment(3);
    assert_eq!(std::mem::size_of_val(&*a), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*b), 0, "noop counter is ZST");
}

// --- histogram (MetricRegistry) ---

#[test]
fn test_histogram_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("latency_ms");
    hist.record(10.0);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_histogram_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("");
    hist.record(1.0);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_histogram_same_name_multiple_times_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let h1 = registry.histogram("h");
    let h2 = registry.histogram("h");
    h1.record(1.0);
    h2.record(2.0);
    assert_eq!(std::mem::size_of_val(&*h1), 0, "noop histogram is ZST");
    assert_eq!(std::mem::size_of_val(&*h2), 0, "noop histogram is ZST");
}

// --- gauge (MetricRegistry) ---

#[test]
fn test_gauge_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("queue.depth");
    gauge.set(7.0);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is ZST");
}

#[test]
fn test_gauge_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("");
    gauge.set(0.0);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is ZST");
}

#[test]
fn test_gauge_all_instruments_together_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let c = registry.counter("c");
    let h = registry.histogram("h");
    let g = registry.gauge("g");
    c.increment(1);
    h.record(1.0);
    g.set(1.0);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
    assert_eq!(std::mem::size_of_val(&*h), 0, "noop histogram is ZST");
    assert_eq!(std::mem::size_of_val(&*g), 0, "noop gauge is ZST");
}

#[test]
fn test_metric_registry_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    assert_send_sync(&registry);
    assert_eq!(std::mem::size_of_val(&*registry), 0, "noop metric registry is ZST");
}

#[test]
fn test_metric_registry_returns_dyn_trait_object() {
    let registry: Box<dyn MetricRegistry> = StdObserveFactory::noop_metric_registry();
    let c = registry.counter("total");
    c.increment(1);
    assert_eq!(std::mem::size_of_val(&*c), 0, "noop counter is ZST");
}
