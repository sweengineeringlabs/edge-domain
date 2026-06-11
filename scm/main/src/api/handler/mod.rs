//! `Handler` theme — execution-unit port contracts and domain-layer decorators.

pub mod errors;
pub mod event_emitting_handler;
pub mod in_process_handler_registry;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use traits::Handler;
pub use traits::HandlerFactory;
pub use traits::HandlerRegistry;
pub use types::EchoHandler;
pub use types::EventEmittingHandler;
pub use types::InProcessHandlerRegistry;
pub use types::RequestContext;
pub use types::RequestContextBuilder;
