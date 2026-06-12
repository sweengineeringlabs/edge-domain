//! `Repository` theme — data access contracts for domain entities.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::RepositoryError;
pub use traits::{InMemoryRepository, QueryableRepository, Repository, RepositoryFactory};
pub use types::{Page, Spec};
