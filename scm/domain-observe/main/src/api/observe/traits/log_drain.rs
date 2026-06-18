//! `LogDrain` — structured log emission contract.

use crate::api::observe::types::LogRecord;

/// Receives structured log records emitted by domain handlers.
pub trait LogDrain: Send + Sync {
    /// Emit a structured log record.
    fn emit(&self, record: LogRecord);
}
