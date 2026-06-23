use super::noop_handler_tracer::NoopHandlerTracer;
use super::noop_log_drain::NoopLogDrain;
use super::noop_metric_registry::NoopMetricRegistry;
use crate::api::HandlerTracer;
use crate::api::LogDrain;
use crate::api::MetricRegistry;
use crate::api::ObserverContext;

pub(crate) struct NoopObserverContext {
    tracer: NoopHandlerTracer,
    drain: NoopLogDrain,
    metrics: NoopMetricRegistry,
}

impl NoopObserverContext {
    pub(crate) fn new() -> Self {
        Self {
            tracer: NoopHandlerTracer::new(),
            drain: NoopLogDrain::new(),
            metrics: NoopMetricRegistry::new(),
        }
    }
}

impl ObserverContext for NoopObserverContext {
    fn tracer(&self) -> &dyn HandlerTracer {
        &self.tracer
    }

    fn drain(&self) -> &dyn LogDrain {
        &self.drain
    }

    fn metrics(&self) -> &dyn MetricRegistry {
        &self.metrics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_noop_observer_context_happy() {
        let ctx = NoopObserverContext::new();
        let span = ctx.tracer().start_span("h", "op");
        span.finish();
        assert_eq!(std::mem::size_of_val(&*span), 0, "noop observer context is a ZST");
    }

    #[test]
    fn test_tracer_start_span_no_panic_error() {
        let ctx = NoopObserverContext::new();
        let span = ctx.tracer().start_span("h", "op");
        span.finish();
        assert_eq!(std::mem::size_of_val(&*span), 0, "tracer span is a ZST");
    }

    #[test]
    fn test_metrics_counter_no_panic_edge() {
        let ctx = NoopObserverContext::new();
        let counter = ctx.metrics().counter("m");
        counter.increment(1);
        assert_eq!(std::mem::size_of_val(&*counter), 0, "metric counter is a ZST");
    }
}
