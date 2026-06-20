//! `impl ObserveBootstrap for StdObserveFactory`.

use super::{NoopHandlerTracer, NoopLogDrain, NoopMetricRegistry};
use crate::api::HandlerTracer;
use crate::api::LogDrain;
use crate::api::MetricRegistry;
use crate::api::ObserveBootstrap;
use crate::api::StdObserveFactory;

impl ObserveBootstrap for StdObserveFactory {
    fn build_handler_tracer(&self) -> Box<dyn HandlerTracer> {
        Box::new(NoopHandlerTracer::new())
    }

    fn build_metric_registry(&self) -> Box<dyn MetricRegistry> {
        Box::new(NoopMetricRegistry::new())
    }

    fn build_log_drain(&self) -> Box<dyn LogDrain> {
        Box::new(NoopLogDrain::new())
    }
}
