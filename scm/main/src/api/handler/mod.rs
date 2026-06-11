//! `Handler` theme — execution-unit port contracts and domain-layer decorators.

pub mod errors;
pub mod event_emitting_handler;
pub mod in_process_handler_registry;
pub mod request;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use request::{RequestContext, RequestContextBuilder};
pub use traits::{Handler, HandlerFactory, HandlerRegistry};
pub use types::{EchoHandler, EventEmittingHandler, InProcessHandlerRegistry};
