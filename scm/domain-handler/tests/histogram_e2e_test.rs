//! SAF facade tests — `Histogram` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    HandlerError, Histogram, HistogramRecordRequest, HistogramRecordResponse,
};

struct OkHistogram;
impl Histogram for OkHistogram {
    fn record(
        &self,
        _req: HistogramRecordRequest,
    ) -> Result<HistogramRecordResponse, HandlerError> {
        Ok(HistogramRecordResponse)
    }
}

struct FailingHistogram;
impl Histogram for FailingHistogram {
    fn record(
        &self,
        _req: HistogramRecordRequest,
    ) -> Result<HistogramRecordResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed(
            "histogram unavailable".into(),
        ))
    }
}

/// @covers: Histogram::record — success
#[test]
fn test_record_ok_histogram_returns_ok_happy() {
    assert_eq!(
        OkHistogram.record(HistogramRecordRequest { value: 12.3 }),
        Ok(HistogramRecordResponse)
    );
}

/// @covers: Histogram::record — failure propagates
#[test]
fn test_record_failing_histogram_returns_err_error() {
    assert!(FailingHistogram
        .record(HistogramRecordRequest { value: 1.0 })
        .is_err());
}

/// @covers: Histogram::record — zero value accepted
#[test]
fn test_record_zero_value_returns_ok_edge() {
    assert_eq!(
        OkHistogram.record(HistogramRecordRequest { value: 0.0 }),
        Ok(HistogramRecordResponse)
    );
}
