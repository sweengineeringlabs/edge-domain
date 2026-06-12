//! SAF — handler service facade.

mod handler;

pub use crate::api::handler::EchoHandler;
pub use crate::api::handler::Handler;
pub use crate::api::handler::HandlerError;
pub use crate::api::handler::HandlerFactory;
pub use crate::api::handler::HandlerProvider;
pub use crate::api::handler::HandlerRegistry;
pub use crate::api::handler::InProcessHandlerRegistry;
pub use crate::api::handler::RequestContext;
pub use crate::api::handler::RequestContextBuilder;
