//! Integration tests for `Clock`, `SystemClock`, and `FixedClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_domain::{Clock, FixedClock, SystemClock};
use edge_domain_clock::NowRequest;

/// @covers: Clock::now (SystemClock)
#[test]
fn test_now_system_clock_returns_time_within_current_window_happy() {
    let before = SystemTime::now();
    let t = SystemClock.now(NowRequest).unwrap().instant;
    let after = SystemTime::now();
    assert!(t >= before);
    assert!(t <= after);
}

/// @covers: Clock::now (FixedClock)
#[test]
fn test_now_fixed_clock_returns_exact_configured_instant_happy() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let clock = FixedClock::new(instant);
    assert_eq!(clock.now(NowRequest).unwrap().instant, instant);
}

/// @covers: Clock::now (SystemClock monotonicity)
#[test]
fn test_now_system_clock_successive_calls_do_not_go_backwards_error() {
    let t1 = SystemClock.now(NowRequest).unwrap().instant;
    let t2 = SystemClock.now(NowRequest).unwrap().instant;
    assert!(t2 >= t1);
}

/// @covers: Clock::now (FixedClock at UNIX_EPOCH)
#[test]
fn test_now_fixed_clock_at_unix_epoch_returns_epoch_edge() {
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH);
    assert_eq!(
        clock.now(NowRequest).unwrap().instant,
        SystemTime::UNIX_EPOCH
    );
}

/// @covers: Clock::now (FixedClock determinism)
#[test]
fn test_now_fixed_clock_repeated_calls_return_same_instant_edge() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
    let clock = FixedClock::new(instant);
    let t1 = clock.now(NowRequest).unwrap().instant;
    let t2 = clock.now(NowRequest).unwrap().instant;
    assert_eq!(t1, instant);
    assert_eq!(t2, instant);
}
