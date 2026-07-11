//! Integration tests — `LocalGaugeAdapter`, exercised indirectly via `MetricRegistry::gauge`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    GaugeLookupRequest, GaugeSetRequest, GaugeSetResponse, MetricsRequest, ObserverContext,
};
use edge_domain_observer::StdObserveFactory;

/// @covers: LocalGaugeAdapter — set delegates to the real bridged gauge
#[test]
fn test_gauge_adapter_set_via_noop_returns_ok_happy() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let gauge = metrics
        .gauge(GaugeLookupRequest { name: "g".into() })
        .unwrap()
        .gauge;
    assert_eq!(
        gauge.set(GaugeSetRequest { value: 42.0 }),
        Ok(GaugeSetResponse)
    );
}

/// @covers: LocalGaugeAdapter — a negative value is accepted without panicking
#[test]
fn test_gauge_adapter_negative_value_returns_ok_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let gauge = metrics
        .gauge(GaugeLookupRequest { name: "g".into() })
        .unwrap()
        .gauge;
    assert_eq!(
        gauge.set(GaugeSetRequest { value: -1.5 }),
        Ok(GaugeSetResponse)
    );
}

/// @covers: LocalGaugeAdapter — a zero value round-trips
#[test]
fn test_gauge_adapter_zero_value_returns_ok_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let gauge = metrics
        .gauge(GaugeLookupRequest { name: "g".into() })
        .unwrap()
        .gauge;
    assert_eq!(
        gauge.set(GaugeSetRequest { value: 0.0 }),
        Ok(GaugeSetResponse)
    );
}
