//! SAF — handler service facade.
#[cfg(not(feature = "handler"))]
pub use crate::api::EchoHandler;
#[cfg(not(feature = "handler"))]
pub use crate::api::Handler;
#[cfg(not(feature = "handler"))]
pub use crate::api::HandlerError;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const HANDLER_SVC: () = ();
