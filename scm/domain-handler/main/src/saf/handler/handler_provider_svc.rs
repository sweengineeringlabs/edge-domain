/// Service name constant for the `HandlerProvider` trait.
pub const HANDLER_PROVIDER_SVC: &str = "handler_provider";

pub(crate) use crate::api::EchoHandler;
pub use crate::api::HandlerProvider;
pub(crate) use crate::api::InProcessHandlerRegistry;
