//! Blanket bridges from `edge_domain_observer`'s traits to their local
//! `domain-handler` decoupling boundaries (SEA `no_foreign_type`).

use edge_domain_observer as obs;

use crate::api::HandlerError;
use crate::api::{Counter, CounterLookupRequest, CounterLookupResponse};
use crate::api::{DrainRequest, DrainResponse};
use crate::api::{Gauge, GaugeLookupRequest, GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse};
use crate::api::{
    Histogram, HistogramLookupRequest, HistogramLookupResponse, HistogramRecordRequest,
    HistogramRecordResponse,
};
use crate::api::{HandlerTracer, MetricRegistry, ObserverContext};
use crate::api::{IncrementRequest, IncrementResponse};
use crate::api::{LogDrain, LogEmitRequest, LogEmitResponse};
use crate::api::{MetricsRequest, MetricsResponse};
use crate::api::{Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest, SpanFinishResponse};
use crate::api::{SpanStartRequest, SpanStartResponse, TracerRequest, TracerResponse};

/// Converts a real [`obs::ObserveError`] into the local [`HandlerError`].
trait IntoHandlerError {
    fn into_handler_error(self) -> HandlerError;
}

impl IntoHandlerError for obs::ObserveError {
    fn into_handler_error(self) -> HandlerError {
        HandlerError::ExecutionFailed(self.to_string())
    }
}

impl<T: obs::Span + ?Sized> Span for T {
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        obs::Span::record(
            self,
            obs::SpanAnnotationRequest {
                key: req.key,
                value: req.value,
            },
        )
        .map(|_| SpanAnnotationResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }

    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        obs::Span::finish(self, obs::SpanFinishRequest)
            .map(|_| SpanFinishResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

impl<T: obs::Counter + ?Sized> Counter for T {
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        obs::Counter::increment(self, obs::IncrementRequest { delta: req.delta })
            .map(|_| IncrementResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

impl<T: obs::Gauge + ?Sized> Gauge for T {
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError> {
        obs::Gauge::set(self, obs::GaugeSetRequest { value: req.value })
            .map(|_| GaugeSetResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

impl<T: obs::Histogram + ?Sized> Histogram for T {
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, HandlerError> {
        obs::Histogram::record(self, obs::HistogramRecordRequest { value: req.value })
            .map(|_| HistogramRecordResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

impl<T: obs::LogDrain + ?Sized> LogDrain for T {
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError> {
        obs::LogDrain::emit(
            self,
            obs::LogEmitRequest {
                level: req.level,
                handler_id: req.handler_id,
                message: req.message,
            },
        )
        .map(|_| LogEmitResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }
}

/// Adapter wrapping an owned real `Span` as a local [`Span`].
struct SpanAdapter(Box<dyn obs::Span>);

impl Span for SpanAdapter {
    fn record(&self, req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        obs::Span::record(
            self.0.as_ref(),
            obs::SpanAnnotationRequest {
                key: req.key,
                value: req.value,
            },
        )
        .map(|_| SpanAnnotationResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }

    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        obs::Span::finish(self.0.as_ref(), obs::SpanFinishRequest)
            .map(|_| SpanFinishResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

/// Adapter wrapping an owned real `Counter` as a local [`Counter`].
struct CounterAdapter(Box<dyn obs::Counter>);

impl Counter for CounterAdapter {
    fn increment(&self, req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        obs::Counter::increment(self.0.as_ref(), obs::IncrementRequest { delta: req.delta })
            .map(|_| IncrementResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

/// Adapter wrapping an owned real `Gauge` as a local [`Gauge`].
struct GaugeAdapter(Box<dyn obs::Gauge>);

impl Gauge for GaugeAdapter {
    fn set(&self, req: GaugeSetRequest) -> Result<GaugeSetResponse, HandlerError> {
        obs::Gauge::set(self.0.as_ref(), obs::GaugeSetRequest { value: req.value })
            .map(|_| GaugeSetResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

/// Adapter wrapping an owned real `Histogram` as a local [`Histogram`].
struct HistogramAdapter(Box<dyn obs::Histogram>);

impl Histogram for HistogramAdapter {
    fn record(&self, req: HistogramRecordRequest) -> Result<HistogramRecordResponse, HandlerError> {
        obs::Histogram::record(self.0.as_ref(), obs::HistogramRecordRequest { value: req.value })
            .map(|_| HistogramRecordResponse)
            .map_err(IntoHandlerError::into_handler_error)
    }
}

impl<T: obs::HandlerTracer + ?Sized> HandlerTracer for T {
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError> {
        let resp = obs::HandlerTracer::start_span(
            self,
            obs::SpanStartRequest {
                handler_id: req.handler_id,
                operation: req.operation,
            },
        )
        .map_err(IntoHandlerError::into_handler_error)?;
        Ok(SpanStartResponse {
            span: Box::new(SpanAdapter(resp.span)),
        })
    }
}

impl<T: obs::MetricRegistry + ?Sized> MetricRegistry for T {
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, HandlerError> {
        let resp = obs::MetricRegistry::counter(self, obs::CounterLookupRequest { name: req.name })
            .map_err(IntoHandlerError::into_handler_error)?;
        Ok(CounterLookupResponse {
            counter: Box::new(CounterAdapter(resp.counter)),
        })
    }

    fn histogram(&self, req: HistogramLookupRequest) -> Result<HistogramLookupResponse, HandlerError> {
        let resp = obs::MetricRegistry::histogram(self, obs::HistogramLookupRequest { name: req.name })
            .map_err(IntoHandlerError::into_handler_error)?;
        Ok(HistogramLookupResponse {
            histogram: Box::new(HistogramAdapter(resp.histogram)),
        })
    }

    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, HandlerError> {
        let resp = obs::MetricRegistry::gauge(self, obs::GaugeLookupRequest { name: req.name })
            .map_err(IntoHandlerError::into_handler_error)?;
        Ok(GaugeLookupResponse {
            gauge: Box::new(GaugeAdapter(resp.gauge)),
        })
    }
}

/// Adapter wrapping a borrowed real `HandlerTracer` reference as a local [`HandlerTracer`].
struct HandlerTracerRef<'a>(&'a dyn obs::HandlerTracer);

impl HandlerTracer for HandlerTracerRef<'_> {
    fn start_span(&self, req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError> {
        let resp = obs::HandlerTracer::start_span(
            self.0,
            obs::SpanStartRequest {
                handler_id: req.handler_id,
                operation: req.operation,
            },
        )
        .map_err(IntoHandlerError::into_handler_error)?;
        Ok(SpanStartResponse {
            span: Box::new(SpanAdapter(resp.span)),
        })
    }
}

/// Adapter wrapping a borrowed real `LogDrain` reference as a local [`LogDrain`].
struct LogDrainRef<'a>(&'a dyn obs::LogDrain);

impl LogDrain for LogDrainRef<'_> {
    fn emit(&self, req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError> {
        obs::LogDrain::emit(
            self.0,
            obs::LogEmitRequest {
                level: req.level,
                handler_id: req.handler_id,
                message: req.message,
            },
        )
        .map(|_| LogEmitResponse)
        .map_err(IntoHandlerError::into_handler_error)
    }
}

/// Adapter wrapping a borrowed real `MetricRegistry` reference as a local [`MetricRegistry`].
struct MetricRegistryRef<'a>(&'a dyn obs::MetricRegistry);

impl MetricRegistry for MetricRegistryRef<'_> {
    fn counter(&self, req: CounterLookupRequest) -> Result<CounterLookupResponse, HandlerError> {
        let resp = obs::MetricRegistry::counter(self.0, obs::CounterLookupRequest { name: req.name })
            .map_err(IntoHandlerError::into_handler_error)?;
        Ok(CounterLookupResponse {
            counter: Box::new(CounterAdapter(resp.counter)),
        })
    }

    fn histogram(&self, req: HistogramLookupRequest) -> Result<HistogramLookupResponse, HandlerError> {
        let resp =
            obs::MetricRegistry::histogram(self.0, obs::HistogramLookupRequest { name: req.name })
                .map_err(IntoHandlerError::into_handler_error)?;
        Ok(HistogramLookupResponse {
            histogram: Box::new(HistogramAdapter(resp.histogram)),
        })
    }

    fn gauge(&self, req: GaugeLookupRequest) -> Result<GaugeLookupResponse, HandlerError> {
        let resp = obs::MetricRegistry::gauge(self.0, obs::GaugeLookupRequest { name: req.name })
            .map_err(IntoHandlerError::into_handler_error)?;
        Ok(GaugeLookupResponse {
            gauge: Box::new(GaugeAdapter(resp.gauge)),
        })
    }
}

impl<T: obs::ObserverContext + ?Sized> ObserverContext for T {
    fn tracer(&self, _req: TracerRequest) -> Result<TracerResponse<'_>, HandlerError> {
        let resp = obs::ObserverContext::tracer(self, obs::TracerRequest).map_err(IntoHandlerError::into_handler_error)?;
        Ok(TracerResponse {
            tracer: Box::new(HandlerTracerRef(resp.tracer)),
        })
    }

    fn drain(&self, _req: DrainRequest) -> Result<DrainResponse<'_>, HandlerError> {
        let resp = obs::ObserverContext::drain(self, obs::DrainRequest).map_err(IntoHandlerError::into_handler_error)?;
        Ok(DrainResponse {
            drain: Box::new(LogDrainRef(resp.drain)),
        })
    }

    fn metrics(&self, _req: MetricsRequest) -> Result<MetricsResponse<'_>, HandlerError> {
        let resp = obs::ObserverContext::metrics(self, obs::MetricsRequest).map_err(IntoHandlerError::into_handler_error)?;
        Ok(MetricsResponse {
            metrics: Box::new(MetricRegistryRef(resp.metrics)),
        })
    }
}

impl<T: obs::ObserverContext + ?Sized> ObserverContext for crate::api::ObserverContextAdapter<'_, T> {
    fn tracer(&self, req: TracerRequest) -> Result<TracerResponse<'_>, HandlerError> {
        ObserverContext::tracer(self.0, req)
    }

    fn drain(&self, req: DrainRequest) -> Result<DrainResponse<'_>, HandlerError> {
        ObserverContext::drain(self.0, req)
    }

    fn metrics(&self, req: MetricsRequest) -> Result<MetricsResponse<'_>, HandlerError> {
        ObserverContext::metrics(self.0, req)
    }
}

#[cfg(test)]
mod tests {
    use edge_domain_observer::StdObserveFactory;

    use super::*;

    #[test]
    fn test_span_bridge_record_via_noop_returns_ok_happy() {
        let observer = StdObserveFactory::noop_observer_context();
        let tracer = ObserverContext::tracer(observer.as_ref(), TracerRequest).unwrap().tracer;
        let span = tracer
            .start_span(SpanStartRequest {
                handler_id: "h".into(),
                operation: "op".into(),
            })
            .unwrap()
            .span;
        assert_eq!(
            span.record(SpanAnnotationRequest {
                key: "k".into(),
                value: "v".into(),
            }),
            Ok(SpanAnnotationResponse)
        );
    }

    #[test]
    fn test_observer_context_adapter_bridges_erased_reference_edge() {
        let observer = StdObserveFactory::noop_observer_context();
        let adapter = crate::api::ObserverContextAdapter(observer.as_ref());
        let span = ObserverContext::tracer(&adapter, TracerRequest)
            .unwrap()
            .tracer
            .start_span(SpanStartRequest {
                handler_id: "h".into(),
                operation: "op".into(),
            })
            .unwrap()
            .span;
        assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
    }

    #[test]
    fn test_counter_bridge_increment_via_noop_returns_ok_error() {
        let observer = StdObserveFactory::noop_observer_context();
        let metrics = ObserverContext::metrics(observer.as_ref(), MetricsRequest).unwrap().metrics;
        let counter = metrics
            .counter(CounterLookupRequest { name: "c".into() })
            .unwrap()
            .counter;
        assert_eq!(counter.increment(IncrementRequest { delta: 1 }), Ok(IncrementResponse));
    }

    #[test]
    fn test_observer_context_drain_via_noop_emits_ok_edge() {
        let observer = StdObserveFactory::noop_observer_context();
        let drain = ObserverContext::drain(observer.as_ref(), DrainRequest).unwrap().drain;
        assert_eq!(
            drain.emit(LogEmitRequest {
                level: "info".into(),
                handler_id: "h".into(),
                message: "m".into(),
            }),
            Ok(LogEmitResponse)
        );
    }
}
