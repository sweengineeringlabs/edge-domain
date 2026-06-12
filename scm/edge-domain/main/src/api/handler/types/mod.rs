//! Handler theme — concrete types and value objects defined in `edge-domain`.

pub mod echo_handler;
pub mod in_process_handler_registry;

pub use echo_handler::EchoHandler;
pub use in_process_handler_registry::InProcessHandlerRegistry;
