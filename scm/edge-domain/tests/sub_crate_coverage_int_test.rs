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
    let _ = std::mem::size_of::<SystemClock>();
    let _ = std::mem::size_of::<ValidatorError>();
    let _ = std::mem::size_of::<PolicyError>();
    let _ = std::mem::size_of::<CommandError>();
    let _ = std::mem::size_of::<QueryError>();
    let _ = std::mem::size_of::<SnapshotError>();
    let _ = std::mem::size_of::<ServiceError>();
    let _ = std::mem::size_of::<RepositoryError>();
    let _ = std::mem::size_of::<HandlerError>();
    let _ = std::mem::size_of::<EventError>();
    let _ = std::mem::size_of::<ProjectionError>();
    let _ = std::mem::size_of::<SagaError>();
    let _ = std::mem::size_of::<SecurityError>();
}
