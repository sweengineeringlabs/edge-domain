//! Handler module — trait, registry, and request context.

#[allow(clippy::module_inception)]
pub mod handler;
pub mod echo_handler;
pub mod handler_registry;
pub mod request;

pub use echo_handler::EchoHandler;
pub use handler::Handler;
pub use request::RequestContext;
pub use request::RequestContextBuilder;
