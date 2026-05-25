//! Handler module — trait, registry, and request context.

pub mod echo_handler;
#[allow(clippy::module_inception)]
pub mod handler;
pub mod handler_registry;
pub mod request;

pub use handler::Handler;
pub use crate::api::types::RequestContext;
pub use crate::api::types::RequestContextBuilder;
