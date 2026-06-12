//! SAF — saga registry service facade.
#[cfg(not(feature = "saga"))]
pub use crate::api::saga::SagaRegistry;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SAGA_REGISTRY_SVC: () = ();
