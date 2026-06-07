//! `Repository` theme — data access contracts for domain entities.

pub mod error;
pub mod traits;
pub mod types;

pub use error::RepositoryError;
pub use traits::{QueryableRepository, Repository};
pub use types::{InMemoryRepository, Page, Spec};
