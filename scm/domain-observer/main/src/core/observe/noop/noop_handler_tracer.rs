use super::noop_span::NoopSpan;
use crate::api::HandlerTracer;
use crate::api::Span;

pub(crate) struct NoopHandlerTracer;

impl NoopHandlerTracer {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl HandlerTracer for NoopHandlerTracer {
    fn start_span(&self, handler_id: &str, operation: &str) -> Box<dyn Span> {
        let _ = (handler_id, operation);
        Box::new(NoopSpan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_noop_handler_tracer_happy() {
        let t = NoopHandlerTracer::new();
        assert_eq!(std::mem::size_of_val(&t), 0);
    }

    #[test]
    fn test_start_span_empty_ids_no_panic_error() {
        let t = NoopHandlerTracer::new();
        let span = t.start_span("", "");
        span.finish();
        assert_eq!(std::mem::size_of_val(&t), 0);
    }

    #[test]
    fn test_start_span_multiple_calls_independent_edge() {
        let t = NoopHandlerTracer::new();
        t.start_span("a", "op1").finish();
        t.start_span("b", "op2").finish();
        assert_eq!(std::mem::size_of_val(&t), 0);
    }
}
