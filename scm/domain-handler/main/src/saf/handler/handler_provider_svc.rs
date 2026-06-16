/// Service name constant for the `HandlerProvider` trait.
pub const HANDLER_PROVIDER_SVC: &str = "handler_provider";

pub use crate::api::handler::EchoHandler;
pub use crate::api::handler::HandlerProvider;
pub use crate::api::handler::InProcessHandlerRegistry;
