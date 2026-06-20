//! SAF facade tests — `ClockBootstrap` constructors.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_domain_clock::{Clock, ClockBootstrap};

struct TestClocks;
impl ClockBootstrap for TestClocks {}

/// @covers: ClockBootstrap::system — returns a usable wall clock
#[test]
fn test_system_returns_usable_wall_clock_happy() {
    let before = SystemTime::now();
    let clock = TestClocks::system();
    let t = clock.now();
    // SystemTime is not monotonic on Windows, so only assert forward progress
    // from a point captured strictly before the call.
    assert!(t >= before);
}

/// @covers: ClockBootstrap::system — does not return a past-stuck clock
#[test]
fn test_system_is_not_stuck_in_past_error() {
    let clock = TestClocks::system();
    assert!(clock.now() > SystemTime::UNIX_EPOCH);
}

/// @covers: ClockBootstrap::system — zero-sized marker
#[test]
fn test_system_returns_system_clock_type_edge() {
    let clock = TestClocks::system();
    assert_eq!(std::mem::size_of_val(&clock), 0);
}

/// @covers: ClockBootstrap::fixed — reports the pinned time
#[test]
fn test_fixed_reports_pinned_time_happy() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let clock = TestClocks::fixed(instant);
    assert_eq!(clock.now(), instant);
}

/// @covers: ClockBootstrap::fixed — does not advance between calls
#[test]
fn test_fixed_does_not_advance_error() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(999);
    let clock = TestClocks::fixed(instant);
    assert_eq!(clock.now(), clock.now());
}

/// @covers: ClockBootstrap::fixed — anchors at UNIX_EPOCH
#[test]
fn test_fixed_unix_epoch_anchor_edge() {
    let clock = TestClocks::fixed(SystemTime::UNIX_EPOCH);
    assert_eq!(clock.now(), SystemTime::UNIX_EPOCH);
}
