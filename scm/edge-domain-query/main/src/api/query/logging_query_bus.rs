//! `LoggingQueryBus` — SEA Rule 121 api/core mirror.

/// A `QueryBus` decorator that records dispatch outcomes via `tracing`.
pub type LoggingQueryBus<R> =
    crate::api::query::types::logging_query_bus::LoggingQueryBus<R>;
