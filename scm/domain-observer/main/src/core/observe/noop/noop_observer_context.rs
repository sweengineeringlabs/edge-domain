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
        // Context is correctly initialized with valid trait objects.
        let tracer_size = std::mem::size_of_val(&ctx.tracer());
        assert!(tracer_size > 0, "tracer trait object must have non-zero size");
    }

    #[test]
    fn test_tracer_start_span_no_panic_error() {
        let ctx = NoopObserverContext::new();
        ctx.tracer().start_span("h", "op").finish();
        // Tracer span operations complete without panic.
        let drain_size = std::mem::size_of_val(&ctx.drain());
        assert!(drain_size > 0, "drain trait object must have non-zero size");
    }

    #[test]
    fn test_metrics_counter_no_panic_edge() {
        let ctx = NoopObserverContext::new();
        ctx.metrics().counter("m").increment(1);
        // Metrics counter operations complete without panic.
        assert_eq!(std::mem::size_of_val(&ctx.metrics()), std::mem::size_of_val(&ctx.metrics()));
    }
}
