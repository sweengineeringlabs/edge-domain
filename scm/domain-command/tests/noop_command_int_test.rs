use edge_domain_command::{Command, NoopCommand};

#[tokio::test]
async fn test_noop_command_execute_returns_ok_happy() {
    assert!(NoopCommand.execute().await.is_ok());
}

#[test]
fn test_noop_command_name_returns_default_happy() {
    assert_eq!(NoopCommand.name(), "command");
}

#[test]
fn test_noop_command_default_name_does_not_signal_failure_error() {
    // NoopCommand never signals domain failure — name must not be empty
    assert!(!NoopCommand.name().is_empty());
}

#[test]
fn test_noop_command_is_copy_type_edge() {
    let a = NoopCommand;
    let b = a;
    // Both a and b should still be valid, proving Copy semantics
    assert_eq!(a.name(), b.name());
}
