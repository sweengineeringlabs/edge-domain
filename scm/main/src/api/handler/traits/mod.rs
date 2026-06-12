//! Handler theme — port contracts owned by the domain.

pub mod handler;
pub mod handler_factory;
pub mod handler_registry;

pub use handler::Handler;
pub use handler_factory::HandlerFactory;
pub use handler_registry::HandlerRegistry;

pub use crate::api::handler::types::EchoHandler;
pub use crate::api::handler::types::InProcessHandlerRegistry;
pub use crate::api::handler::types::RequestContextBuilder;
