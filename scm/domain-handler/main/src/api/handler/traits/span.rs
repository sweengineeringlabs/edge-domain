//! `Span` — local decoupling boundary for a traced unit of work.

use crate::api::handler::errors::HandlerError;
use crate::api::handler::dto::{
    SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest, SpanFinishResponse,
};

/// A tracing span produced by a [`HandlerTracer`](super::HandlerTracer).
///
/// Declared locally so `api/` never references `edge_application_observer::Span`
/// directly in a type position (SEA `no_foreign_type`). Any real `Span`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait Span: Send + Sync {
    /// Attach a key-value annotation to this span.
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError>;

    /// Mark this span as finished.
    fn finish(&self, req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError>;
}
