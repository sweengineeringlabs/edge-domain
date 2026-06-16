/// Service name constant for the `HandlerFactory` trait.
pub const HANDLER_FACTORY_SVC: &str = "handler_factory";

pub use crate::api::handler::HandlerFactory;
pub use crate::api::handler::NoopHandlerFactory;
