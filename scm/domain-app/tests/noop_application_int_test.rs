//! Integration tests — `NoopApplication` (api noop type).

use edge_domain_app::{Application, AppError, NoopApplication};
use futures::executor::block_on;

/// @covers: NoopApplication — default name is "application"
#[test]
fn test_noop_application_name_returns_application_happy() {
    assert_eq!(NoopApplication.name(), "application");
}

/// @covers: NoopApplication — run() never returns an error
#[test]
fn test_noop_application_run_never_errors_error() {
    let result: Result<(), AppError> = block_on(NoopApplication.run());
    assert_eq!(result, Ok(()));
}

/// @covers: NoopApplication — is Copy; clone behaves identically
#[test]
fn test_noop_application_copy_preserves_name_edge() {
    let original = NoopApplication;
    let copy = original;
    assert_eq!(original.name(), copy.name());
    assert_eq!(block_on(copy.run()), Ok(()));
}
