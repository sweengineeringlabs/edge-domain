//! [`SpanAnnotationRequest`] — request to attach a key-value annotation to a span.

/// Request to attach a key-value annotation to a [`Span`](crate::api::handler::traits::Span).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpanAnnotationRequest {
    /// The annotation key.
    pub key: String,
    /// The annotation value.
    pub value: String,
}
