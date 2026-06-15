//! SAF — saga store service facade.
#[cfg(not(feature = "saga"))]
pub use crate::api::saga::SagaStore;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SAGA_STORE_SVC: () = ();
