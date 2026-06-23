use edge_domain_observer::StdObserveFactory;

// --- set ---

#[test]
fn test_set_positive_value_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("queue.depth");
    gauge.set(100.0);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is a ZST");
}

#[test]
fn test_set_negative_value_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("queue.depth");
    gauge.set(-50.0);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is a ZST");
}

#[test]
fn test_set_zero_value_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("queue.depth");
    gauge.set(0.0);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "noop gauge is a ZST");
}

#[test]
fn test_gauge_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    let gauge = registry.gauge("g");
    assert_send_sync(&gauge);
    assert_eq!(std::mem::size_of_val(&*gauge), 0, "gauge is Send+Sync ZST");
}
