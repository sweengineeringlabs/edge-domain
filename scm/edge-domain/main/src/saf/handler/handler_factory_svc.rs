//! SAF — handler factory service facade.
#[cfg(not(feature = "handler"))]
pub use crate::api::HandlerFactory;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const HANDLER_FACTORY_SVC: () = ();
