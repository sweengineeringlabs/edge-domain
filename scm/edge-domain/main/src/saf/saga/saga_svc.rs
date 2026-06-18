//! SAF — saga service facade.
#[cfg(not(feature = "saga"))]
pub use crate::api::Saga;
#[cfg(not(feature = "saga"))]
pub use crate::api::SagaError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SAGA_SVC: () = ();
