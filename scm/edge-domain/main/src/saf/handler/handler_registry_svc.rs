//! SAF — handler registry service facade.
#[cfg(not(feature = "handler"))]
pub use crate::api::HandlerRegistry;
#[cfg(not(feature = "handler"))]
pub use crate::api::InProcessHandlerRegistry;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const HANDLER_REGISTRY_SVC: () = ();
