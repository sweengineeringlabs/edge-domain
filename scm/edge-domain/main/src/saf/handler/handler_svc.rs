//! SAF — handler service facade.
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::EchoHandler;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::Handler;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::HandlerError;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::RequestContext;
#[cfg(not(feature = "handler"))]
pub use crate::api::handler::RequestContextBuilder;
/// SAF module anchor — satisfies arch-audit rule 221.
pub const HANDLER_SVC: () = ();
