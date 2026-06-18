//! `HandlerTracer` — tracing contract for domain handlers.

use super::Span;

/// Opens tracing spans for a domain handler execution.
///
/// Spans produced here are children of the `pipeline.stage` span created by
/// `edge-dispatch` for every handler invocation.
pub trait HandlerTracer: Send + Sync {
    /// Start a new span named `operation` for handler `handler_id`.
    fn start_span(&self, handler_id: &str, operation: &str) -> Box<dyn Span>;
}
