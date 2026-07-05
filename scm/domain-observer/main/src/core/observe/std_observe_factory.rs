//! `impl ObserveBootstrap for StdObserveFactory`.

use super::{NoopHandlerTracer, NoopLogDrain, NoopMetricRegistry};
use crate::api::HandlerTracer;
use crate::api::HandlerTracerBuildRequest;
use crate::api::HandlerTracerBuildResponse;
use crate::api::LogDrain;
use crate::api::LogDrainBuildRequest;
use crate::api::LogDrainBuildResponse;
use crate::api::MetricRegistry;
use crate::api::MetricRegistryBuildRequest;
use crate::api::MetricRegistryBuildResponse;
use crate::api::ObserveBootstrap;
use crate::api::ObserveError;
use crate::api::StdObserveFactory;

impl ObserveBootstrap for StdObserveFactory {
    fn build_handler_tracer(
        &self,
        _req: HandlerTracerBuildRequest,
    ) -> Result<HandlerTracerBuildResponse, ObserveError> {
        Ok(HandlerTracerBuildResponse {
            tracer: Box::new(NoopHandlerTracer::new()) as Box<dyn HandlerTracer>,
        })
    }

    fn build_metric_registry(
        &self,
        _req: MetricRegistryBuildRequest,
    ) -> Result<MetricRegistryBuildResponse, ObserveError> {
        Ok(MetricRegistryBuildResponse {
            metric_registry: Box::new(NoopMetricRegistry::new()) as Box<dyn MetricRegistry>,
        })
    }

    fn build_log_drain(
        &self,
        _req: LogDrainBuildRequest,
    ) -> Result<LogDrainBuildResponse, ObserveError> {
        Ok(LogDrainBuildResponse {
            log_drain: Box::new(NoopLogDrain::new()) as Box<dyn LogDrain>,
        })
    }
}
