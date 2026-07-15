//! Integration tests — `LocalCounterAdapter`, exercised indirectly via `MetricRegistry::counter`
//! (its only public construction path).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    CounterLookupRequest, IncrementRequest, IncrementResponse, MetricsRequest, ObserverContext,
};
use edge_application_observer::StdObserveFactory;

/// @covers: LocalCounterAdapter — increment delegates to the real bridged counter
#[test]
fn test_counter_adapter_increment_via_noop_returns_ok_happy() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let counter = metrics
        .counter(CounterLookupRequest { name: "c".into() })
        .unwrap()
        .counter;
    assert_eq!(
        counter.increment(IncrementRequest { delta: 1 }),
        Ok(IncrementResponse)
    );
}

/// @covers: LocalCounterAdapter — a large delta is accepted without panicking
#[test]
fn test_counter_adapter_large_delta_returns_ok_error() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let counter = metrics
        .counter(CounterLookupRequest { name: "c".into() })
        .unwrap()
        .counter;
    assert_eq!(
        counter.increment(IncrementRequest { delta: 1_000_000 }),
        Ok(IncrementResponse)
    );
}

/// @covers: LocalCounterAdapter — an empty counter name still resolves to a usable counter
#[test]
fn test_counter_adapter_empty_name_returns_usable_counter_edge() {
    let observer = StdObserveFactory::noop_observer_context();
    let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest)
        .unwrap()
        .metrics;
    let counter = metrics
        .counter(CounterLookupRequest {
            name: String::new(),
        })
        .unwrap()
        .counter;
    assert_eq!(
        counter.increment(IncrementRequest { delta: 0 }),
        Ok(IncrementResponse)
    );
}
