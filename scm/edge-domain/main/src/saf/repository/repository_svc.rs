//! SAF — repository service facade.
#[cfg(not(feature = "repository"))]
pub use crate::api::InMemoryRepository;
#[cfg(not(feature = "repository"))]
pub use crate::api::Page;
#[cfg(not(feature = "repository"))]
pub use crate::api::QueryableRepository;
#[cfg(not(feature = "repository"))]
pub use crate::api::Repository;
#[cfg(not(feature = "repository"))]
pub use crate::api::RepositoryError;
#[cfg(not(feature = "repository"))]
pub use crate::api::Spec;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const REPOSITORY_SVC: () = ();
