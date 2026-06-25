//! `Handler` theme — request/response execution port contracts.

pub mod errors;
pub mod in_process_handler_registry;
pub mod registry_bridge;
pub mod service_handler;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use registry_bridge::RegistryBridge;
pub use service_handler::ServiceHandler;
pub use traits::{Handler, HandlerBootstrap, HandlerProvider, HandlerRegistry, IntoHandler, ServiceBridge, Validator};
pub use types::{EchoHandler, HandlerContext, NoopHandlerFactory, StdRegistryBridge};
