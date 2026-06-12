//! [`HandlerProvider`] — constructor contract for handler infrastructure types.

use crate::api::handler::types::echo_handler::EchoHandler;
use crate::api::handler::types::in_process_handler_registry::InProcessHandlerRegistry;
use crate::api::handler::types::request_context_builder::RequestContextBuilder;

/// Factory trait for the standard handler-infrastructure types.
pub trait HandlerProvider {
    /// Construct a string-typed [`EchoHandler`] with the given `id` and `pattern`.
    fn echo_handler(id: &str, pattern: &str) -> EchoHandler<String> {
        EchoHandler::new(id, pattern)
    }

    /// Construct the [`InProcessHandlerRegistry`] marker type.
    fn in_process_registry() -> InProcessHandlerRegistry {
        InProcessHandlerRegistry
    }

    /// Construct a fresh [`RequestContextBuilder`].
    fn request_context_builder() -> RequestContextBuilder {
        RequestContextBuilder::new()
    }
}
