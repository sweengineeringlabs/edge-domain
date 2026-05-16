//! Handler module — trait, registry, and request context.

pub mod echo_handler;
#[allow(clippy::module_inception)]
pub mod handler;
pub mod handler_registry;
pub mod request;

pub use echo_handler::EchoHandler;
pub use handler::Handler;
pub use request::RequestContext;
pub use request::RequestContextBuilder;
