use super::noop_handler_tracer::NoopHandlerTracer;
use super::noop_log_drain::NoopLogDrain;
use super::noop_metric_registry::NoopMetricRegistry;
use crate::api::HandlerTracer;
use crate::api::LogDrain;
use crate::api::MetricRegistry;
use crate::api::ObserveContext;

pub(crate) struct NoopObserveContext {
    tracer: NoopHandlerTracer,
    drain: NoopLogDrain,
    metrics: NoopMetricRegistry,
}

impl NoopObserveContext {
    pub(crate) fn new() -> Self {
        Self {
            tracer: NoopHandlerTracer::new(),
            drain: NoopLogDrain::new(),
            metrics: NoopMetricRegistry::new(),
        }
    }
}

impl ObserveContext for NoopObserveContext {
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
    fn test_new_creates_noop_observe_context_happy() {
        let _ = NoopObserveContext::new();
    }

    #[test]
    fn test_tracer_start_span_no_panic_error() {
        let ctx = NoopObserveContext::new();
        ctx.tracer().start_span("h", "op").finish();
    }

    #[test]
    fn test_metrics_counter_no_panic_edge() {
        let ctx = NoopObserveContext::new();
        ctx.metrics().counter("m").increment(1);
    }
}
