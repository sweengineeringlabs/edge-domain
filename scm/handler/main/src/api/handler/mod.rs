//! `Handler` theme — request/response execution port contracts.

pub mod command_bus_adapter;
pub mod dto;
mod echo_handler;
pub mod errors;
pub mod in_process_handler_registry;
mod observability_bridge;
pub mod observer_context_adapter;
pub mod traits;
pub mod vo;

pub use command_bus_adapter::CommandBusAdapter;
pub use dto::{
    CommandDispatchRequest, CommandExecutionRequest, CommandNameRequest, CommandNameResponse,
    CounterLookupRequest, CounterLookupResponse, DeregisterHandlerRequest,
    DeregisterHandlerResponse, DrainRequest, DrainResponse, EmptinessRequest, EmptinessResponse,
    ExecutionRequest, GaugeLookupRequest, GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse,
    HandlerLookupRequest, HandlerLookupResponse, HealthCheckRequest, HealthCheckResponse,
    HistogramLookupRequest, HistogramLookupResponse, HistogramRecordRequest,
    HistogramRecordResponse, IdRequest, IdResponse, IncrementRequest, IncrementResponse,
    LenRequest, LenResponse, ListIdsRequest, ListIdsResponse, LogEmitRequest, LogEmitResponse,
    MetricsRequest, MetricsResponse, PatternRequest, PatternResponse, RegisterHandlerRequest,
    RegisterHandlerResponse, Request, Response, SpanAnnotationRequest, SpanAnnotationResponse,
    SpanFinishRequest, SpanFinishResponse, SpanStartRequest, SpanStartResponse, TracerRequest,
    TracerResponse,
};
pub use echo_handler::EchoHandler;
pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use observer_context_adapter::ObserverContextAdapter;
pub use traits::{
    Command, CommandBus, Counter, Gauge, Handler, HandlerRegistry, HandlerTracer, Histogram,
    LogDrain, MetricRegistry, ObserverContext, SecurityPrincipal, Span,
};
pub use vo::HandlerContext;
