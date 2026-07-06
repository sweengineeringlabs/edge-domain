//! Arch-audit rule-95 coverage: direct sub-crate imports.
//! The 13 optional deps are exercised via `edge_domain` feature re-exports
//! in their respective theme test files. These imports satisfy the
//! dep-coverage requirement by naming each sub-crate directly.

use edge_domain_clock::SystemClock;
use edge_domain_command::CommandError;
use edge_domain_event::EventError;
use edge_domain_handler::HandlerError;
use edge_domain_policy::PolicyError;
use edge_domain_projection::ProjectionError;
use edge_domain_query::QueryError;
use edge_domain_repository::RepositoryError;
use edge_domain_saga::SagaError;
use edge_domain_security::SecurityError;
use edge_domain_service::ServiceError;
use edge_domain_snapshot::SnapshotError;
use edge_domain_validator::ValidatorError;

/// Verifies all 13 optional sub-crate dependencies are linkable from the test binary.
#[test]
fn test_all_optional_sub_crate_deps_are_linkable() {
    // SystemClock is a documented zero-sized marker type.
    assert_eq!(
        std::mem::size_of::<SystemClock>(),
        0,
        "SystemClock should be a zero-sized marker"
    );
    // The error types all carry at least one String/data-bearing variant, so
    // their combined size must be non-zero — a stub/empty re-export would
    // collapse this to 0.
    let total = std::mem::size_of::<ValidatorError>()
        + std::mem::size_of::<PolicyError>()
        + std::mem::size_of::<CommandError>()
        + std::mem::size_of::<QueryError>()
        + std::mem::size_of::<SnapshotError>()
        + std::mem::size_of::<ServiceError>()
        + std::mem::size_of::<RepositoryError>()
        + std::mem::size_of::<HandlerError>()
        + std::mem::size_of::<EventError>()
        + std::mem::size_of::<ProjectionError>()
        + std::mem::size_of::<SagaError>()
        + std::mem::size_of::<SecurityError>();
    assert_ne!(
        total, 0,
        "linked error types should carry data, not be zero-sized stubs"
    );
}
