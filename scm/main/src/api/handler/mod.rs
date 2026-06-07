//! `Handler` theme — execution-unit port contracts and domain-layer decorators.

pub mod error;
pub mod traits;
pub mod types;

pub use error::HandlerError;
pub use traits::Handler;
pub use traits::HandlerFactory;
pub use traits::HandlerRegistry;
pub use types::EchoHandler;
pub use types::EventEmittingHandler;
pub use types::RequestContext;
pub use types::RequestContextBuilder;
