//! [`SpanStartResponse`] — wrapper for a newly opened tracing span.

use crate::api::Span;

/// Result of [`HandlerTracer::start_span`](crate::api::HandlerTracer::start_span).
pub struct SpanStartResponse {
    /// The newly opened span.
    pub span: Box<dyn Span>,
}
