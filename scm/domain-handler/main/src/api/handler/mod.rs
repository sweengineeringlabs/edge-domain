//! `Handler` theme — request/response execution port contracts.

pub mod errors;
pub mod in_process_handler_registry;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use traits::{Handler, HandlerFactory, HandlerProvider, HandlerRegistry};
pub use types::{EchoHandler, HandlerContext, NoopHandlerFactory};
