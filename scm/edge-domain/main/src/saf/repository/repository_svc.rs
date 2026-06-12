//! SAF — repository service facade.
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::InMemoryRepository;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::Page;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::QueryableRepository;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::Repository;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::RepositoryError;
#[cfg(not(feature = "repository"))]
pub use crate::api::repository::Spec;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const REPOSITORY_SVC: () = ();
