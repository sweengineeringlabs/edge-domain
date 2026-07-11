//! SAF facade tests — `ObserverContext` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_handler::{
    CounterLookupRequest, DrainRequest, DrainResponse, GaugeLookupRequest, HandlerError,
    HandlerTracer, HistogramLookupRequest, IncrementRequest, IncrementResponse, LogDrain,
    LogEmitRequest, LogEmitResponse, MetricRegistry, MetricsRequest, MetricsResponse,
    ObserverContext, Span, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse, SpanStartRequest, SpanStartResponse, TracerRequest, TracerResponse,
};

struct StubSpan;
impl Span for StubSpan {
    fn record(&self, _req: SpanAnnotationRequest) -> Result<SpanAnnotationResponse, HandlerError> {
        Ok(SpanAnnotationResponse)
    }
    fn finish(&self, _req: SpanFinishRequest) -> Result<SpanFinishResponse, HandlerError> {
        Ok(SpanFinishResponse)
    }
}

struct StubTracer;
impl HandlerTracer for StubTracer {
    fn start_span(&self, _req: SpanStartRequest) -> Result<SpanStartResponse, HandlerError> {
        Ok(SpanStartResponse {
            span: Box::new(StubSpan),
        })
    }
}

struct StubDrain;
impl LogDrain for StubDrain {
    fn emit(&self, _req: LogEmitRequest) -> Result<LogEmitResponse, HandlerError> {
        Ok(LogEmitResponse)
    }
}

struct StubCounter;
impl edge_domain_handler::Counter for StubCounter {
    fn increment(&self, _req: IncrementRequest) -> Result<IncrementResponse, HandlerError> {
        Ok(IncrementResponse)
    }
}

struct StubRegistry;
impl MetricRegistry for StubRegistry {
    fn counter(
        &self,
        _req: CounterLookupRequest,
    ) -> Result<edge_domain_handler::CounterLookupResponse, HandlerError> {
        Ok(edge_domain_handler::CounterLookupResponse {
            counter: Box::new(StubCounter),
        })
    }
    fn histogram(
        &self,
        _req: HistogramLookupRequest,
    ) -> Result<edge_domain_handler::HistogramLookupResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("not needed".into()))
    }
    fn gauge(
        &self,
        _req: GaugeLookupRequest,
    ) -> Result<edge_domain_handler::GaugeLookupResponse, HandlerError> {
        Err(HandlerError::ExecutionFailed("not needed".into()))
    }
}

struct OkObserver;
impl ObserverContext for OkObserver {
    fn tracer(&self, _req: TracerRequest) -> Result<TracerResponse<'_>, HandlerError> {
        Ok(TracerResponse {
            tracer: Box::new(StubTracer),
        })
    }
    fn drain(&self, _req: DrainRequest) -> Result<DrainResponse<'_>, HandlerError> {
        Ok(DrainResponse {
            drain: Box::new(StubDrain),
        })
    }
    fn metrics(&self, _req: MetricsRequest) -> Result<MetricsResponse<'_>, HandlerError> {
        Ok(MetricsResponse {
            metrics: Box::new(StubRegistry),
        })
    }
}

struct FailingObserver;
impl ObserverContext for FailingObserver {
    fn tracer(&self, _req: TracerRequest) -> Result<TracerResponse<'_>, HandlerError> {
        Err(HandlerError::ExecutionFailed("observer offline".into()))
    }
    fn drain(&self, _req: DrainRequest) -> Result<DrainResponse<'_>, HandlerError> {
        Err(HandlerError::ExecutionFailed("observer offline".into()))
    }
    fn metrics(&self, _req: MetricsRequest) -> Result<MetricsResponse<'_>, HandlerError> {
        Err(HandlerError::ExecutionFailed("observer offline".into()))
    }
}

/// @covers: ObserverContext::tracer — success
#[test]
fn test_tracer_ok_observer_returns_tracer_happy() {
    let tracer = OkObserver
        .tracer(TracerRequest)
        .expect("tracer should succeed")
        .tracer;
    let span = tracer
        .start_span(SpanStartRequest {
            handler_id: "h".into(),
            operation: "op".into(),
        })
        .unwrap()
        .span;
    assert_eq!(span.finish(SpanFinishRequest), Ok(SpanFinishResponse));
}

/// @covers: ObserverContext::tracer — failure propagates
#[test]
fn test_tracer_failing_observer_returns_err_error() {
    assert!(FailingObserver.tracer(TracerRequest).is_err());
}

/// @covers: ObserverContext::tracer — callable repeatedly
#[test]
fn test_tracer_called_repeatedly_returns_tracer_edge() {
    let observer = OkObserver;
    for _ in 0..2 {
        let span = observer
            .tracer(TracerRequest)
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
}

/// @covers: ObserverContext::drain — success
#[test]
fn test_drain_ok_observer_emits_ok_happy() {
    let drain = OkObserver
        .drain(DrainRequest)
        .expect("drain should succeed")
        .drain;
    assert_eq!(
        drain.emit(LogEmitRequest {
            level: "info".into(),
            handler_id: "h".into(),
            message: "m".into(),
        }),
        Ok(LogEmitResponse)
    );
}

/// @covers: ObserverContext::drain — failure propagates
#[test]
fn test_drain_failing_observer_returns_err_error() {
    assert!(FailingObserver.drain(DrainRequest).is_err());
}

/// @covers: ObserverContext::drain — callable repeatedly
#[test]
fn test_drain_called_repeatedly_returns_drain_edge() {
    let observer = OkObserver;
    for _ in 0..2 {
        let drain = observer.drain(DrainRequest).unwrap().drain;
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

/// @covers: ObserverContext::metrics — success
#[test]
fn test_metrics_ok_observer_returns_registry_happy() {
    let metrics = OkObserver
        .metrics(MetricsRequest)
        .expect("metrics should succeed")
        .metrics;
    let counter = metrics
        .counter(CounterLookupRequest { name: "c".into() })
        .unwrap()
        .counter;
    assert_eq!(
        counter.increment(IncrementRequest { delta: 1 }),
        Ok(IncrementResponse)
    );
}

/// @covers: ObserverContext::metrics — failure propagates
#[test]
fn test_metrics_failing_observer_returns_err_error() {
    assert!(FailingObserver.metrics(MetricsRequest).is_err());
}

/// @covers: ObserverContext::metrics — callable repeatedly
#[test]
fn test_metrics_called_repeatedly_returns_registry_edge() {
    let observer = OkObserver;
    for _ in 0..2 {
        let metrics = observer.metrics(MetricsRequest).unwrap().metrics;
        let counter = metrics
            .counter(CounterLookupRequest { name: "c".into() })
            .unwrap()
            .counter;
        assert_eq!(
            counter.increment(IncrementRequest { delta: 1 }),
            Ok(IncrementResponse)
        );
    }
}

/// @covers: ObserverContext::wrap — wraps an already type-erased real `ObserverContext`
#[test]
fn test_wrap_erased_reference_bridges_happy() {
    let observer = edge_domain_observer::StdObserveFactory::noop_observer_context();
    let adapter = OkObserver::wrap(observer.as_ref());
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

/// @covers: ObserverContext::wrap — wrapped erased reference bridges drain emission
#[test]
fn test_wrap_erased_reference_propagates_errors_error() {
    let observer = edge_domain_observer::StdObserveFactory::noop_observer_context();
    let adapter = OkObserver::wrap(observer.as_ref());
    let drain = ObserverContext::drain(&adapter, DrainRequest)
        .unwrap()
        .drain;
    assert_eq!(
        drain.emit(LogEmitRequest {
            level: "info".into(),
            handler_id: "h".into(),
            message: "m".into(),
        }),
        Ok(LogEmitResponse)
    );
}

/// @covers: ObserverContext::wrap — adapter reusable across multiple calls
#[test]
fn test_wrap_adapter_reusable_edge() {
    let observer = edge_domain_observer::StdObserveFactory::noop_observer_context();
    let adapter = OkObserver::wrap(observer.as_ref());
    for _ in 0..2 {
        let metrics = ObserverContext::metrics(&adapter, MetricsRequest)
            .unwrap()
            .metrics;
        let counter = metrics
            .counter(CounterLookupRequest { name: "c".into() })
            .unwrap()
            .counter;
        assert_eq!(
            counter.increment(IncrementRequest { delta: 1 }),
            Ok(IncrementResponse)
        );
    }
}
