//! SAF — handler registry service facade.
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::HandlerRegistry;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::InProcessHandlerRegistry;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const HANDLER_REGISTRY_SVC: () = ();
