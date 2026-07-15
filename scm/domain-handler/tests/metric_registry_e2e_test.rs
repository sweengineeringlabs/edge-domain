//! SAF facade tests — `MetricRegistry` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{
    Counter, CounterLookupRequest, CounterLookupResponse, Gauge, GaugeLookupRequest,
    GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, HandlerError, Histogram,
    HistogramLookupRequest, HistogramLookupResponse, HistogramRecordRequest,
    HistogramRecordResponse, IncrementRequest, IncrementResponse, MetricRegistry,
};

struct StubCounter;
impl Counter for StubCounter {
    fn increment(&self, _req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        Ok(IncrementResponse)
    }
}

struct StubGauge;
impl Gauge for StubGauge {
    fn set(&self, _req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError> {
        Ok(GaugeSetResponse)
    }
}

struct StubHistogram;
impl Histogram for StubHistogram {
    fn record(
        &self,
        _req: HistogramRecordRequest,
    ) -> Result<HistogramRecordResponse, HandlerError> {
        Ok(HistogramRecordResponse)
    }
}

struct OkRegistry;
impl MetricRegistry for OkRegistry {
    fn counter(&self, _req: CounterLookupRequest) -> Result<CounterLookupResponse, HandlerError> {
        Ok(CounterLookupResponse {
            counter: Box::new(StubCounter),
        })
    }
    fn histogram(
        &self,
        _req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, HandlerError> {
        Ok(HistogramLookupResponse {
            histogram: Box::new(StubHistogram),
        })
    }
    fn gauge(&self, _req: GaugeLookupRequest) -> Result<GaugeLookupResponse, HandlerError> {
        Ok(GaugeLookupResponse {
            gauge: Box::new(StubGauge),
        })
    }
}

struct FailingRegistry;
impl MetricRegistry for FailingRegistry {
    fn counter(&self, _req: CounterLookupRequest) -> Result<CounterLookupResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("registry offline".into()))
    }
    fn histogram(
        &self,
        _req: HistogramLookupRequest,
    ) -> Result<HistogramLookupResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("registry offline".into()))
    }
    fn gauge(&self, _req: GaugeLookupRequest) -> Result<GaugeLookupResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("registry offline".into()))
    }
}

/// @covers: MetricRegistry::counter — success
#[test]
fn test_counter_ok_registry_returns_counter_happy() {
    let counter = OkRegistry
        .counter(CounterLookupRequest { name: "c".into() })
        .expect("counter should succeed")
        .counter;
    assert_eq!(
        counter.increment(IncrementRequest { delta: 1 }),
        Ok(IncrementResponse)
    );
}

/// @covers: MetricRegistry::counter — failure propagates
#[test]
fn test_counter_failing_registry_returns_err_error() {
    assert!(FailingRegistry
        .counter(CounterLookupRequest { name: "c".into() })
        .is_err());
}

/// @covers: MetricRegistry::counter — empty name accepted
#[test]
fn test_counter_empty_name_returns_counter_edge() {
    let counter = OkRegistry
        .counter(CounterLookupRequest { name: "".into() })
        .expect("counter should succeed")
        .counter;
    assert_eq!(
        counter.increment(IncrementRequest { delta: 1 }),
        Ok(IncrementResponse)
    );
}

/// @covers: MetricRegistry::histogram — success
#[test]
fn test_histogram_ok_registry_returns_histogram_happy() {
    let histogram = OkRegistry
        .histogram(HistogramLookupRequest { name: "h".into() })
        .expect("histogram should succeed")
        .histogram;
    assert_eq!(
        histogram.record(HistogramRecordRequest { value: 1.0 }),
        Ok(HistogramRecordResponse)
    );
}

/// @covers: MetricRegistry::histogram — failure propagates
#[test]
fn test_histogram_failing_registry_returns_err_error() {
    assert!(FailingRegistry
        .histogram(HistogramLookupRequest { name: "h".into() })
        .is_err());
}

/// @covers: MetricRegistry::histogram — empty name accepted
#[test]
fn test_histogram_empty_name_returns_histogram_edge() {
    let histogram = OkRegistry
        .histogram(HistogramLookupRequest { name: "".into() })
        .expect("histogram should succeed")
        .histogram;
    assert_eq!(
        histogram.record(HistogramRecordRequest { value: 1.0 }),
        Ok(HistogramRecordResponse)
    );
}

/// @covers: MetricRegistry::gauge — success
#[test]
fn test_gauge_ok_registry_returns_gauge_happy() {
    let gauge = OkRegistry
        .gauge(GaugeLookupRequest { name: "g".into() })
        .expect("gauge should succeed")
        .gauge;
    assert_eq!(
        gauge.set(GaugeSetRequest { value: 1.0 }),
        Ok(GaugeSetResponse)
    );
}

/// @covers: MetricRegistry::gauge — failure propagates
#[test]
fn test_gauge_failing_registry_returns_err_error() {
    assert!(FailingRegistry
        .gauge(GaugeLookupRequest { name: "g".into() })
        .is_err());
}

/// @covers: MetricRegistry::gauge — empty name accepted
#[test]
fn test_gauge_empty_name_returns_gauge_edge() {
    let gauge = OkRegistry
        .gauge(GaugeLookupRequest { name: "".into() })
        .expect("gauge should succeed")
        .gauge;
    assert_eq!(
        gauge.set(GaugeSetRequest { value: 1.0 }),
        Ok(GaugeSetResponse)
    );
}
