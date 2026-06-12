//! `Handler` theme — request/response execution port contracts.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use traits::{Handler, HandlerFactory, HandlerProvider, HandlerRegistry};
pub use types::{EchoHandler, InProcessHandlerRegistry};
