//! SAF layer — domain public facade.

mod factory;
mod validator;

pub use factory::new_handler_registry;
pub use validator::validate;

pub use crate::api::handler::Handler;
pub use crate::api::handler_error::HandlerError;
pub use crate::api::handler::handler_registry::HandlerRegistry;
pub use crate::api::outbound_registry::OutboundRegistry;
