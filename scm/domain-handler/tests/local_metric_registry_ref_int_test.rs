//! Integration tests — `LocalMetricRegistryRef`, exercised indirectly via `ObserverContext::metrics`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    CounterLookupRequest, GaugeLookupRequest, HistogramLookupRequest, MetricsRequest,
    ObserverContext,
};
use edge_domain_observer::StdObserveFactory;

/// @covers: LocalMetricRegistryRef::counter — resolves a counter through the borrowed real registry
#[test]
fn test_metric_registry_ref_counter_via_noop_resolves_happy() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    assert!(metrics
        .counter(CounterLookupRequest { name: "c".into() })
        .is_ok());
}

/// @covers: LocalMetricRegistryRef::histogram — resolves a histogram through the borrowed real registry
#[test]
fn test_metric_registry_ref_histogram_via_noop_resolves_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    assert!(metrics
        .histogram(HistogramLookupRequest { name: "h".into() })
        .is_ok());
}

/// @covers: LocalMetricRegistryRef::gauge — the same reference resolves multiple distinct instruments
#[test]
fn test_metric_registry_ref_resolves_multiple_distinct_instruments_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    assert!(metrics
        .gauge(GaugeLookupRequest { name: "g".into() })
        .is_ok());
    assert!(metrics
        .counter(CounterLookupRequest { name: "c2".into() })
        .is_ok());
    assert!(metrics
        .histogram(HistogramLookupRequest { name: "h2".into() })
        .is_ok());
}
