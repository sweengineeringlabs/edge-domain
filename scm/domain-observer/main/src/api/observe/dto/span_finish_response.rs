//! [`SpanFinishResponse`] — wrapper for a successfully finished span.

/// Result of [`Span::finish`](crate::api::Span::finish).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpanFinishResponse;
