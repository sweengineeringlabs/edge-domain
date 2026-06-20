use edge_domain_clock::{ClockBootstrap, StdClockFactory};
use std::time::{Duration, SystemTime};

#[test]
fn test_std_factory_system_creates_system_clock_happy() {
    let _ = StdClockFactory::system();
}

#[test]
fn test_std_factory_fixed_returns_pinned_clock_happy() {
    let at = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
    let _ = StdClockFactory::fixed(at);
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let _ = StdClockFactory::std_factory();
}

#[test]
fn test_std_factory_fixed_at_epoch_returns_zero_time_error() {
    use edge_domain_clock::Clock;
    let at = SystemTime::UNIX_EPOCH;
    let c = StdClockFactory::fixed(at);
    assert_eq!(c.now(), SystemTime::UNIX_EPOCH);
}

#[test]
fn test_std_factory_is_copy_type_edge() {
    let f = StdClockFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after move
}
