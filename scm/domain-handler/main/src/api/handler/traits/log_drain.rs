//! `LogDrain` — local decoupling boundary for structured log emission.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::dto::{LogEmitRequest, LogEmitResponse};

/// Receives structured log records emitted by domain handlers.
///
/// Declared locally so `api/` never references `edge_application_observer::LogDrain`
/// directly in a type position (SEA `no_foreign_type`). Any real `LogDrain`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait LogDrain: Send + Sync {
    /// Emit a structured log record.
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError>;
}
