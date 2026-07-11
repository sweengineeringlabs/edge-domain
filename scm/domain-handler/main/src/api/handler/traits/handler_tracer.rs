//! `HandlerTracer` — local decoupling boundary for tracing.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::types::{SpanStartRequest, SpanStartResponse};

/// Opens tracing spans for a domain handler execution.
///
/// Declared locally so `api/` never references `edge_domain_observer::HandlerTracer`
/// directly in a type position (SEA `no_foreign_type`). Any real `HandlerTracer`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait HandlerTracer: Send + Sync {
    /// Start a new span named `operation` for handler `handler_id`.
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError>;
}
