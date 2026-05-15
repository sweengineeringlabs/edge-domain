//! Integration tests for error bridging: domain errors → HandlerError.

use edge_domain::{CommandError, HandlerError, RepositoryError, ServiceError};

/// @covers: From<ServiceError> for HandlerError
#[test]
fn test_service_error_invalid_request_maps_to_handler_invalid_request() {
    let e: HandlerError = ServiceError::InvalidRequest("bad".into()).into();
    assert!(matches!(e, HandlerError::InvalidRequest(_)));
}

/// @covers: From<ServiceError> for HandlerError
#[test]
fn test_service_error_rule_violation_maps_to_handler_failed_precondition() {
    let e: HandlerError = ServiceError::RuleViolation("no".into()).into();
    assert!(matches!(e, HandlerError::FailedPrecondition(_)));
}

/// @covers: From<ServiceError> for HandlerError
#[test]
fn test_service_error_internal_maps_to_handler_execution_failed() {
    let e: HandlerError = ServiceError::Internal("boom".into()).into();
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
}

/// @covers: From<RepositoryError> for HandlerError
#[test]
fn test_repository_error_not_found_maps_to_handler_other() {
    let e: HandlerError = RepositoryError::NotFound("id-1".into()).into();
    assert!(matches!(e, HandlerError::Other(_)));
    assert!(e.to_string().contains("not found"));
}

/// @covers: From<RepositoryError> for HandlerError
#[test]
fn test_repository_error_conflict_maps_to_handler_failed_precondition() {
    let e: HandlerError = RepositoryError::Conflict("dup key".into()).into();
    assert!(matches!(e, HandlerError::FailedPrecondition(_)));
}

/// @covers: From<RepositoryError> for HandlerError
#[test]
fn test_repository_error_unavailable_maps_to_handler_execution_failed() {
    let e: HandlerError = RepositoryError::Unavailable("db down".into()).into();
    assert!(matches!(e, HandlerError::ExecutionFailed(_)));
}

/// @covers: From<CommandError> for HandlerError
#[test]
fn test_command_error_invalid_input_maps_to_handler_invalid_request() {
    let e: HandlerError = CommandError::InvalidInput("missing".into()).into();
    assert!(matches!(e, HandlerError::InvalidRequest(_)));
}

/// @covers: From<CommandError> for HandlerError
#[test]
fn test_command_error_rule_violation_maps_to_handler_failed_precondition() {
    let e: HandlerError = CommandError::RuleViolation("blocked".into()).into();
    assert!(matches!(e, HandlerError::FailedPrecondition(_)));
}

/// @covers: From<CommandError> for HandlerError — ? operator works in handlers
#[test]
fn test_question_mark_operator_converts_command_error_to_handler_error() {
    fn simulate_handler() -> Result<(), HandlerError> {
        let result: Result<(), CommandError> = Err(CommandError::Internal("oops".into()));
        result?;
        Ok(())
    }
    assert!(matches!(simulate_handler(), Err(HandlerError::ExecutionFailed(_))));
}
