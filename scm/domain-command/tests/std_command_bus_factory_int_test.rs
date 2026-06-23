use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};

#[test]
fn test_std_factory_direct_creates_command_bus_happy() {
    let _bus = StdCommandBusFactory::direct();
    let bus: &edge_domain_command::DirectCommandBus = &_bus;
    assert!(std::mem::size_of_val(bus) == 0);
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let factory = StdCommandBusFactory::std_factory();
    let factory_ref: &StdCommandBusFactory = &factory;
    assert!(std::mem::size_of_val(factory_ref) == 0);
}

#[test]
fn test_std_factory_std_factory_is_copy_type_error() {
    let f = StdCommandBusFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after move
    assert!(std::mem::size_of_val(&f) == 0);
}

#[test]
fn test_std_factory_std_factory_is_default_edge() {
    let f = StdCommandBusFactory::default();
    let f_ref: &StdCommandBusFactory = &f;
    assert!(std::mem::size_of_val(f_ref) == 0);
}
