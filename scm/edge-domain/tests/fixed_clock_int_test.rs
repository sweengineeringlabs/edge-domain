//! Integration tests for `FixedClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_domain::{Clock, FixedClock};
use edge_domain_clock::NowRequest;

/// @covers: FixedClock::new, Clock::now
#[test]
fn test_now_returns_exact_configured_instant_happy() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let clock = FixedClock::new(instant);
    assert_eq!(clock.now(NowRequest).unwrap().instant, instant);
}

/// @covers: FixedClock (Clock::now determinism)
#[test]
fn test_now_repeated_calls_return_same_instant_error() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(999);
    let clock = FixedClock::new(instant);
    let first = clock.now(NowRequest).unwrap().instant;
    let second = clock.now(NowRequest).unwrap().instant;
    assert_eq!(first, instant, "first call should return configured instant");
    assert_eq!(second, instant, "second call should return same instant");
}

/// @covers: FixedClock (Clock::now at UNIX_EPOCH)
#[test]
fn test_now_at_unix_epoch_returns_epoch_edge() {
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH);
    assert_eq!(clock.now(NowRequest).unwrap().instant, SystemTime::UNIX_EPOCH);
}
