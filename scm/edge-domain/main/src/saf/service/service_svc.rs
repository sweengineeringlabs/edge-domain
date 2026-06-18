//! SAF — service service facade.
#[cfg(not(feature = "service"))]
pub use crate::api::Service;
#[cfg(not(feature = "service"))]
pub use crate::api::ServiceError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SERVICE_SVC: () = ();
