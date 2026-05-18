//! `Repository` module — data access contracts for domain entities.

pub mod in_memory_repository;
#[allow(clippy::module_inception)]
pub mod repository;

pub use repository::Repository;
