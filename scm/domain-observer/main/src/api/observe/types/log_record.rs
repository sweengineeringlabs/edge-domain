//! `LogRecord` — a structured log entry.

/// A structured log entry emitted to a [`LogDrain`].
///
/// [`LogDrain`]: crate::api::observe::traits::LogDrain
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogRecord {
    /// Severity level as a string (e.g. `"INFO"`, `"WARN"`, `"ERROR"`).
    pub level: String,
    /// Handler that produced this record.
    pub handler_id: String,
    /// Log message body.
    pub message: String,
}

impl LogRecord {
    /// Construct a new `LogRecord`.
    pub fn new(
        level: impl Into<String>,
        handler_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level: level.into(),
            handler_id: handler_id.into(),
            message: message.into(),
        }
    }
}
