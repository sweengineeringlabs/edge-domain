//! SAF facade tests — `Gauge` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_handler::{Gauge, GaugeSetRequest, GaugeSetResponse, HandlerError};

struct OkGauge;
impl Gauge for OkGauge {
    fn set(&self, _req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError> {
        Ok(GaugeSetResponse)
    }
}

struct FailingGauge;
impl Gauge for FailingGauge {
    fn set(&self, _req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("gauge unavailable".into()))
    }
}

/// @covers: Gauge::set — success
#[test]
fn test_set_ok_gauge_returns_ok_happy() {
    assert_eq!(
        OkGauge.set(GaugeSetRequest { value: 1.5 }),
        Ok(GaugeSetResponse)
    );
}

/// @covers: Gauge::set — failure propagates
#[test]
fn test_set_failing_gauge_returns_err_error() {
    assert!(FailingGauge.set(GaugeSetRequest { value: 1.0 }).is_err());
}

/// @covers: Gauge::set — negative values accepted
#[test]
fn test_set_negative_value_returns_ok_edge() {
    assert_eq!(
        OkGauge.set(GaugeSetRequest { value: -1.0 }),
        Ok(GaugeSetResponse)
    );
}
