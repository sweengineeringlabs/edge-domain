use edge_domain_observer::StdObserveFactory;

// --- increment ---

#[test]
fn test_increment_positive_delta_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("requests.total");
    counter.increment(1);
}

#[test]
fn test_increment_max_value_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("requests.total");
    counter.increment(u64::MAX);
}

#[test]
fn test_increment_zero_delta_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("requests.total");
    counter.increment(0);
}

#[test]
fn test_counter_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    let counter = registry.counter("c");
    assert_send_sync(&counter);
}
