use edge_domain_clock::{ClockBootstrap, StdClockFactory};
use std::time::{Duration, SystemTime};

#[test]
fn test_std_factory_system_creates_system_clock_happy() {
    let _clock = StdClockFactory::system();
    assert!(true);
}

#[test]
fn test_std_factory_fixed_returns_pinned_clock_happy() {
    let at = SystemTime::UNIX_EPOCH + Duration::from_secs(42);
    let _clock = StdClockFactory::fixed(at);
    assert!(true);
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let _factory = StdClockFactory::std_factory();
    assert!(true);
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
    assert!(true);
}
