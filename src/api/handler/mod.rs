//! Handler module — trait, registry, and request context.

pub mod handler;
pub mod handler_registry;
pub mod request_context;

pub use handler::Handler;
pub use request_context::RequestContext;
