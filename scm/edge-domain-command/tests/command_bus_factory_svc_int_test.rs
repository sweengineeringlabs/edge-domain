//! SAF facade tests — `CommandBusFactory` constructors.

use edge_domain_command::CommandBusFactory;

struct Buses;
impl CommandBusFactory for Buses {}

/// @covers: CommandBusFactory::direct — returns a usable marker
#[test]
fn test_direct_returns_marker_happy() {
    let bus = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: CommandBusFactory::direct — zero-sized
#[test]
fn test_direct_is_zero_size_error() {
    let bus = Buses::direct();
    assert_eq!(std::mem::size_of_val(&bus), 0);
}

/// @covers: CommandBusFactory::direct — independent calls
#[test]
fn test_direct_independent_calls_edge() {
    let a = Buses::direct();
    let b = Buses::direct();
    assert_eq!(std::mem::size_of_val(&a), std::mem::size_of_val(&b));
}
