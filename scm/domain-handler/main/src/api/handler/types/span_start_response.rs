//! [`SpanStartResponse`] — wrapper for a newly opened tracing span.

use crate::api::handler::traits::Span;

/// Result of [`HandlerTracer::start_span`](crate::api::handler::traits::HandlerTracer::start_span).
pub struct SpanStartResponse {
    /// The newly opened span.
    pub span: Box<dyn Span>,
}
