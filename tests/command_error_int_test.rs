//! Integration tests for `CommandError`.

use edge_domain::CommandError;

/// @covers: CommandError
#[test]
fn test_command_error_invalid_input_display_contains_message() {
    let e = CommandError::InvalidInput("missing field".into());
    assert!(e.to_string().contains("missing field"));
}

/// @covers: CommandError
#[test]
fn test_command_error_rule_violation_display_contains_message() {
    let e = CommandError::RuleViolation("quota exceeded".into());
    assert!(e.to_string().contains("quota exceeded"));
}

/// @covers: CommandError
#[test]
fn test_command_error_not_found_display_contains_message() {
    let e = CommandError::NotFound("order-99".into());
    assert!(e.to_string().contains("order-99"));
}

/// @covers: CommandError
#[test]
fn test_command_error_internal_display_contains_message() {
    let e = CommandError::Internal("db timeout".into());
    assert!(e.to_string().contains("db timeout"));
}

/// @covers: CommandError
#[test]
fn test_command_error_variants_are_distinct() {
    assert!(matches!(
        CommandError::InvalidInput("x".into()),
        CommandError::InvalidInput(_)
    ));
    assert!(matches!(
        CommandError::RuleViolation("x".into()),
        CommandError::RuleViolation(_)
    ));
    assert!(matches!(
        CommandError::NotFound("x".into()),
        CommandError::NotFound(_)
    ));
    assert!(matches!(
        CommandError::Internal("x".into()),
        CommandError::Internal(_)
    ));
}
