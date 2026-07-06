//! `LogDrain` — structured log emission contract.

use crate::api::observe::errors::ObserveError;
use crate::api::observe::types::{LogEmitRequest, LogEmitResponse};

/// Receives structured log records emitted by domain handlers.
pub trait LogDrain: Send + Sync {
    /// Emit a structured log record.
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, ObserveError>;
}
