//! Integration tests for the `ClockFactory` SAF facade.

use std::time::{Duration, SystemTime};

use edge_domain::{Clock, ClockFactory, SystemClock};

struct TestClocks;
impl ClockFactory for TestClocks {}

// --- ClockFactory::system ---

/// @covers ClockFactory::system — happy path: returns a usable wall clock
#[test]
fn test_system_returns_usable_wall_clock_happy() {
    let clock = TestClocks::system();
    let t = clock.now();
    let now = SystemTime::now();
    let delta = now.duration_since(t).unwrap_or(Duration::ZERO);
    assert!(delta.as_secs() < 10, "system clock must return a recent time");
}

/// @covers ClockFactory::system — error: successive calls return non-decreasing times
#[test]
fn test_system_is_not_stuck_in_past_error() {
    let before = SystemTime::now();
    let clock = TestClocks::system();
    let reported = clock.now();
    let after = SystemTime::now();
    assert!(reported >= before, "system clock must not precede call time");
    assert!(reported <= after, "system clock must not exceed call time");
}

/// @covers ClockFactory::system — edge: returned type is SystemClock
#[test]
fn test_system_returns_system_clock_type_edge() {
    let _: SystemClock = TestClocks::system();
}

// --- ClockFactory::fixed ---

/// @covers ClockFactory::fixed — happy path: returned clock reports the pinned time
#[test]
fn test_fixed_reports_pinned_time_happy() {
    use edge_domain::FixedClock;
    let pinned = SystemTime::UNIX_EPOCH;
    let clock: FixedClock = TestClocks::fixed(pinned);
    assert_eq!(clock.now(), pinned);
}

/// @covers ClockFactory::fixed — error: fixed clock does not advance
#[test]
fn test_fixed_does_not_advance_error() {
    let at = SystemTime::UNIX_EPOCH;
    let clock = TestClocks::fixed(at);
    std::thread::sleep(Duration::from_millis(1));
    assert_eq!(clock.now(), at, "fixed clock must not advance after sleep");
}

/// @covers ClockFactory::fixed — edge: UNIX_EPOCH is a valid anchor point
#[test]
fn test_fixed_unix_epoch_anchor_edge() {
    let clock = TestClocks::fixed(SystemTime::UNIX_EPOCH);
    assert_eq!(clock.now(), SystemTime::UNIX_EPOCH);
}
