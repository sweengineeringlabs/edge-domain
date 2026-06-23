use edge_domain_observer::{MetricRegistry, StdObserveFactory};

// --- counter (MetricRegistry) ---

#[test]
fn test_counter_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("requests.total");
    counter.increment(1);
}

#[test]
fn test_counter_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("");
    counter.increment(1);
}

#[test]
fn test_counter_multiple_instruments_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry.counter("a").increment(1);
    registry.counter("b").increment(2);
    registry.counter("a").increment(3);
}

// --- histogram (MetricRegistry) ---

#[test]
fn test_histogram_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("latency_ms");
    hist.record(10.0);
}

#[test]
fn test_histogram_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("");
    hist.record(1.0);
}

#[test]
fn test_histogram_same_name_multiple_times_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry.histogram("h").record(1.0);
    registry.histogram("h").record(2.0);
}

// --- gauge (MetricRegistry) ---

#[test]
fn test_gauge_named_metric_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("queue.depth");
    gauge.set(7.0);
}

#[test]
fn test_gauge_empty_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("");
    gauge.set(0.0);
}

#[test]
fn test_gauge_all_instruments_together_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry.counter("c").increment(1);
    registry.histogram("h").record(1.0);
    registry.gauge("g").set(1.0);
}

#[test]
fn test_metric_registry_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    assert_send_sync(&registry);
}

#[test]
fn test_metric_registry_returns_dyn_trait_object() {
    let registry: Box<dyn MetricRegistry> = StdObserveFactory::noop_metric_registry();
    registry.counter("total").increment(1);
}
