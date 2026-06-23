//! `Span` — a single unit of traced work.

/// A tracing span produced by [`HandlerTracer`].
///
/// [`HandlerTracer`]: super::HandlerTracer
pub trait Span: Send + Sync {
    /// Attach a key-value annotation to this span.
    fn record(&self, key: &str, value: &str);

    /// Mark this span as finished.
    fn finish(&self);
}
