//! `Handler` theme — execution-unit contract, registry, and request context.

pub mod error;
pub mod traits;
pub mod types;
pub mod vo;

pub use error::HandlerError;
pub use traits::Handler;
pub use traits::HandlerRegistry;
pub use types::EchoHandler;
pub use vo::{RequestContext, RequestContextBuilder};
