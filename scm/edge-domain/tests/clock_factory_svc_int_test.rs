//! Integration tests for the `Clock` SAF facade.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_domain::{Clock, SystemClock};
use edge_domain_clock::NowRequest;

// --- SystemClock ---

/// @covers SystemClock — happy path: returns a usable wall clock
#[test]
fn test_system_returns_usable_wall_clock_happy() {
    let clock = SystemClock;
    let t = clock.now(NowRequest).unwrap().instant;
    let now = SystemTime::now();
    let delta = now.duration_since(t).unwrap_or(Duration::ZERO);
    assert!(
        delta.as_secs() < 10,
        "system clock must return a recent time"
    );
}

/// @covers SystemClock — error: successive calls return non-decreasing times
#[test]
fn test_system_is_not_stuck_in_past_error() {
    let before = SystemTime::now();
    let clock = SystemClock;
    let reported = clock.now(NowRequest).unwrap().instant;
    let after = SystemTime::now();
    assert!(
        reported >= before,
        "system clock must not precede call time"
    );
    assert!(reported <= after, "system clock must not exceed call time");
}

/// @covers SystemClock — edge: returned type is SystemClock
#[test]
fn test_system_returns_system_clock_type_edge() {
    let clock: SystemClock = SystemClock;
    let t = clock.now(NowRequest).unwrap().instant;
    assert!(t <= SystemTime::now());
}

// --- FixedClock ---

/// @covers FixedClock::new — happy path: returned clock reports the pinned time
#[test]
fn test_fixed_reports_pinned_time_happy() {
    use edge_domain::FixedClock;
    let pinned = SystemTime::UNIX_EPOCH;
    let clock: FixedClock = FixedClock::new(pinned);
    assert_eq!(clock.now(NowRequest).unwrap().instant, pinned);
}

/// @covers FixedClock::new — error: fixed clock does not advance
#[test]
fn test_fixed_does_not_advance_error() {
    use edge_domain::FixedClock;
    let at = SystemTime::UNIX_EPOCH;
    let clock = FixedClock::new(at);
    std::thread::sleep(Duration::from_millis(1));
    assert_eq!(
        clock.now(NowRequest).unwrap().instant,
        at,
        "fixed clock must not advance after sleep"
    );
}

/// @covers FixedClock::new — edge: UNIX_EPOCH is a valid anchor point
#[test]
fn test_fixed_unix_epoch_anchor_edge() {
    use edge_domain::FixedClock;
    let clock = FixedClock::new(SystemTime::UNIX_EPOCH);
    assert_eq!(clock.now(NowRequest).unwrap().instant, SystemTime::UNIX_EPOCH);
}
