//! # edge-domain-repository
//!
//! The `Repository` port contract — data access with InMemoryRepository, Page, and Spec.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::InMemoryRepository;
pub use saf::Page;
pub use saf::QueryableRepository;
pub use saf::QUERYABLE_REPOSITORY_SVC;
pub use saf::Repository;
pub use saf::REPOSITORY_FACTORY_SVC;
pub use saf::REPOSITORY_SVC;
pub use saf::RepositoryError;
pub use saf::RepositoryBootstrap;
pub use saf::Spec;
pub use saf::SPEC_SVC;
