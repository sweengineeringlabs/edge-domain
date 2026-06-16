//! `LoggingCommandBus` — SEA Rule 121 api/core mirror.

/// A `CommandBus` decorator that records dispatch outcomes via `tracing`.
pub type LoggingCommandBus =
    crate::api::command::types::logging_command_bus::LoggingCommandBus;
