//! API-layer type for the logging command bus.

use std::sync::Arc;

use crate::api::command::CommandBus;

/// A [`CommandBus`] decorator that records each dispatched command and its
/// outcome via `tracing`.
///
/// The concrete `CommandBus` implementation lives in `core::command::logging_command_bus`.
pub struct LoggingCommandBus {
    /// The inner bus that receives the delegated dispatch calls.
    pub inner: Arc<dyn CommandBus>,
}
