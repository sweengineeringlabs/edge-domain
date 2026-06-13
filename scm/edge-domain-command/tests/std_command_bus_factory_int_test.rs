use edge_domain_command::{Command, CommandBusFactory, StdCommandBusFactory};

#[test]
fn test_std_factory_direct_creates_command_bus_happy() {
    let _ = StdCommandBusFactory::direct();
}

#[test]
fn test_noop_command_name_not_empty_happy() {
    assert!(!StdCommandBusFactory::noop_command().name().is_empty());
}

#[test]
fn test_noop_command_name_never_signals_failure_error() {
    assert_eq!(StdCommandBusFactory::noop_command().name(), "command");
}

#[tokio::test]
async fn test_noop_command_execute_always_succeeds_edge() {
    assert!(StdCommandBusFactory::noop_command().execute().await.is_ok());
}

#[test]
fn test_std_factory_std_factory_returns_instance_happy() {
    let _ = StdCommandBusFactory::std_factory();
}

#[test]
fn test_std_factory_std_factory_is_copy_type_error() {
    let f = StdCommandBusFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after move
}

#[test]
fn test_std_factory_std_factory_is_default_edge() {
    let f = StdCommandBusFactory::default();
    let _ = f;
}
