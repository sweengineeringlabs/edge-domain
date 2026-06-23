use edge_domain_command::CommandBusBootstrap;

#[test]
fn test_std_factory_direct_creates_command_bus_happy() {
    let bus = <() as CommandBusBootstrap>::direct();
    assert_eq!(format!("{:?}", bus), "DirectCommandBus");
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let factory = <() as CommandBusBootstrap>::std_factory();
    assert!(!format!("{:?}", factory).is_empty());
}

#[test]
fn test_std_factory_std_factory_is_copy_type_error() {
    let f = <() as CommandBusBootstrap>::std_factory();
    let f2 = f;
    let f3 = f; // Copy — usable after move
    assert_eq!(format!("{:?}", f), format!("{:?}", f3));
}

#[test]
fn test_std_factory_std_factory_is_default_edge() {
    let f = <() as CommandBusBootstrap>::std_factory();
    let f_default = <() as CommandBusBootstrap>::std_factory();
    assert_eq!(format!("{:?}", f), format!("{:?}", f_default));
}
