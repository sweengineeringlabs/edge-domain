//! `Repository` module — data access contracts for domain entities.

pub mod in_memory_repository;
pub mod queryable_repository;
#[allow(clippy::module_inception)]
pub mod repository;
pub mod spec;

pub use queryable_repository::QueryableRepository;
pub use repository::Repository;
pub use spec::Spec;
