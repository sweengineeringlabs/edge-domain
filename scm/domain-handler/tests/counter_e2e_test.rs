//! SAF facade tests — `Counter` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{Counter, HandlerError, IncrementRequest, IncrementResponse};

struct OkCounter;
impl Counter for OkCounter {
    fn increment(&self, _req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        Ok(IncrementResponse)
    }
}

struct FailingCounter;
impl Counter for FailingCounter {
    fn increment(&self, _req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("counter unavailable".into()))
    }
}

/// @covers: Counter::increment — success
#[test]
fn test_increment_ok_counter_returns_ok_happy() {
    assert_eq!(
        OkCounter.increment(IncrementRequest { delta: 1 }),
        Ok(IncrementResponse)
    );
}

/// @covers: Counter::increment — failure propagates
#[test]
fn test_increment_failing_counter_returns_err_error() {
    assert!(FailingCounter
        .increment(IncrementRequest { delta: 1 })
        .is_err());
}

/// @covers: Counter::increment — zero delta is accepted
#[test]
fn test_increment_zero_delta_returns_ok_edge() {
    assert_eq!(
        OkCounter.increment(IncrementRequest { delta: 0 }),
        Ok(IncrementResponse)
    );
}
