//! Rule-222 coverage for [`ClockBootstrap`] trait fns.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, SystemTime};

use edge_domain_clock::{ClockBootstrap, Clock, StdClockFactory};

/// @covers: ClockBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdClockFactory;
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: ClockBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdClockFactory;
    let name1 = f.bootstrap_name();
    let name2 = f.bootstrap_name();
    assert_eq!(
        name1,
        name2,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(name1, "StdClockFactory", "bootstrap_name must return expected identifier");
}

/// @covers: ClockBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn ClockBootstrap = &StdClockFactory;
    let _ = f.bootstrap_name();
}

/// @covers: ClockBootstrap::fixed
#[test]
fn test_fixed_returns_frozen_clock_happy() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(1000);
    let clock = StdClockFactory::fixed(instant);
    assert_eq!(clock.now(), instant, "Fixed clock must return the provided instant");
}

/// @covers: ClockBootstrap::fixed
#[test]
fn test_fixed_deterministic_across_calls_error() {
    let instant = SystemTime::UNIX_EPOCH + Duration::from_secs(2000);
    let clock = StdClockFactory::fixed(instant);
    let t1 = clock.now();
    let t2 = clock.now();
    assert_eq!(t1, t2, "Fixed clock must return same instant on repeated calls");
    assert_eq!(t1, instant, "Fixed clock must match the configured instant");
}

/// @covers: ClockBootstrap::fixed
#[test]
fn test_fixed_with_epoch_boundary_edge() {
    let instant = SystemTime::UNIX_EPOCH;
    let clock = StdClockFactory::fixed(instant);
    assert_eq!(clock.now(), instant, "Fixed clock must handle UNIX_EPOCH");
}
