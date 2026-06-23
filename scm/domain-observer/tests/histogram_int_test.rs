use edge_domain_observer::StdObserveFactory;

// --- record (Histogram) ---

#[test]
fn test_record_positive_value_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("handler.latency_ms");
    hist.record(42.5);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_record_negative_value_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("handler.latency_ms");
    hist.record(-1.0);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_record_zero_value_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("handler.latency_ms");
    hist.record(0.0);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}

#[test]
fn test_histogram_is_send_sync() {
    fn assert_send_sync<T: Send + Sync>(_: &T) {}
    let registry = StdObserveFactory::noop_metric_registry();
    let hist = registry.histogram("h");
    assert_send_sync(&hist);
    assert_eq!(std::mem::size_of_val(&*hist), 0, "noop histogram is ZST");
}
