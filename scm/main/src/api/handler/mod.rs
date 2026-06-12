//! `Handler` theme — execution-unit port contracts and domain-layer decorators.

pub mod errors;
pub mod in_process_handler_registry;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use traits::{
    EchoHandler, Handler, HandlerFactory, HandlerRegistry, InProcessHandlerRegistry,
    RequestContextBuilder,
};
pub use types::RequestContext;
