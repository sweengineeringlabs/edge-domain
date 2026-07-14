//! Integration tests for `SystemClock`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::SystemTime;

use edge_application::{Clock, SystemClock};
use edge_application_clock::NowRequest;

/// @covers: SystemClock (Clock::now)
#[test]
fn test_now_system_clock_returns_time_within_current_window_happy() {
    let before = SystemTime::now();
    let t = SystemClock.now(NowRequest).unwrap().instant;
    let after = SystemTime::now();
    assert!(t >= before);
    assert!(t <= after);
}

/// @covers: SystemClock (Clock::now monotonicity)
#[test]
fn test_now_system_clock_successive_calls_do_not_go_backwards_error() {
    let t1 = SystemClock.now(NowRequest).unwrap().instant;
    let t2 = SystemClock.now(NowRequest).unwrap().instant;
    assert!(t2 >= t1);
}

/// @covers: SystemClock (zero-sized)
#[test]
fn test_system_clock_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<SystemClock>(), 0);
}
