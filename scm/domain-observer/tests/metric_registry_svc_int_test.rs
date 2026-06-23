use edge_domain_observer::{MetricRegistry, StdObserveFactory, METRIC_REGISTRY_SVC};

#[test]
fn test_noop_metric_registry_svc_counter_increments_happy() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry.counter("http.requests").increment(1);
    // Counter was created and incremented without error.
    assert_eq!(std::mem::size_of_val(&*registry), 0);
}

#[test]
fn test_noop_metric_registry_svc_empty_metric_name_error() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry.gauge("").set(-1.0);
    // Empty metric name is handled without error.
    assert_eq!(std::mem::size_of_val(&*registry), 0);
}

#[test]
fn test_noop_metric_registry_svc_all_instrument_types_edge() {
    let registry = StdObserveFactory::noop_metric_registry();
    registry.counter("c").increment(100);
    registry.gauge("g").set(3.14);
    registry.histogram("h").record(0.001);
    // All instruments created without error.
    assert_eq!(std::mem::size_of_val(&*registry), 0);
}

#[test]
fn test_metric_registry_svc_key_namespaced_happy() {
    assert!(METRIC_REGISTRY_SVC.starts_with("edge."));
}

#[test]
fn test_metric_registry_svc_returns_dyn_trait_object() {
    let _registry: Box<dyn MetricRegistry> = StdObserveFactory::noop_metric_registry();
}
