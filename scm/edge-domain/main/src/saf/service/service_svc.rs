//! SAF — service service facade.
#[cfg(not(feature = "service"))]
pub use crate::api::service::Service;
#[cfg(not(feature = "service"))]
pub use crate::api::service::ServiceError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SERVICE_SVC: () = ();
