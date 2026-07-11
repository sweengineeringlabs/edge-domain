//! Integration tests — `LocalHistogramAdapter`, exercised indirectly via `MetricRegistry::histogram`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    HistogramLookupRequest, HistogramRecordRequest, HistogramRecordResponse, MetricsRequest,
    ObserverContext,
};
use edge_domain_observer::StdObserveFactory;

/// @covers: LocalHistogramAdapter — record delegates to the real bridged histogram
#[test]
fn test_histogram_adapter_record_via_noop_returns_ok_happy() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let histogram = metrics
        .histogram(HistogramLookupRequest { name: "h".into() })
        .unwrap()
        .histogram;
    assert_eq!(
        histogram.record(HistogramRecordRequest { value: 3.5 }),
        Ok(HistogramRecordResponse)
    );
}

/// @covers: LocalHistogramAdapter — a negative value is accepted without panicking
#[test]
fn test_histogram_adapter_negative_value_returns_ok_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let histogram = metrics
        .histogram(HistogramLookupRequest { name: "h".into() })
        .unwrap()
        .histogram;
    assert_eq!(
        histogram.record(HistogramRecordRequest { value: -2.0 }),
        Ok(HistogramRecordResponse)
    );
}

/// @covers: LocalHistogramAdapter — repeated recordings are independent
#[test]
fn test_histogram_adapter_repeated_recordings_are_independent_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let histogram = metrics
        .histogram(HistogramLookupRequest { name: "h".into() })
        .unwrap()
        .histogram;
    for v in [0.0, 1.0, 100.0] {
        assert_eq!(
            histogram.record(HistogramRecordRequest { value: v }),
            Ok(HistogramRecordResponse)
        );
    }
}
