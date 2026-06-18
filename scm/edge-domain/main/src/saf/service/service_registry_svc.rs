//! SAF — service registry service facade.
#[cfg(not(feature = "service"))]
pub use crate::api::ServiceRegistryImpl;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const SERVICE_REGISTRY_SVC: () = ();
