//! [`SpanAnnotationResponse`] — wrapper for a successfully recorded span annotation.

/// Result of [`Span::record`](crate::api::handler::traits::Span::record).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpanAnnotationResponse;
