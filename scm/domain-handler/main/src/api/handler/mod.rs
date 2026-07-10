//! `Handler` theme — request/response execution port contracts.

mod command_bridge;
mod echo_handler;
pub mod errors;
pub mod in_process_handler_registry;
mod observability_bridge;
mod service_handler;
pub mod std_registry_bridge;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use traits::{
    Command, CommandBus, Counter, Gauge, Handler, HandlerRegistry, HandlerTracer, Histogram,
    IntoHandler, LogDrain, MetricRegistry, ObserverContext, RegistryBridge, ServiceBridge,
    ServiceHandler, Span, Validator,
};
pub use types::{
    BridgeRequest, BridgeResponse, CommandBusAdapter, CommandDispatchRequest,
    CommandExecutionRequest, CommandNameRequest, CommandNameResponse, CounterLookupRequest,
    CounterLookupResponse, DeregisterHandlerRequest, DeregisterHandlerResponse, DrainRequest,
    DrainResponse, EchoHandler, EmptinessRequest, EmptinessResponse, ExecutionRequest,
    GaugeLookupRequest, GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, HandlerContext,
    HandlerLookupRequest, HandlerLookupResponse, HealthCheckRequest, HealthCheckResponse,
    HistogramLookupRequest, HistogramLookupResponse, HistogramRecordRequest,
    HistogramRecordResponse, IdRequest, IdResponse, IncrementRequest, IncrementResponse,
    IntoHandlerRequest, IntoHandlerResponse, LenRequest, LenResponse, ListIdsRequest,
    ListIdsResponse, LogEmitRequest, LogEmitResponse, MetricsRequest, MetricsResponse,
    ObserverContextAdapter, PatternRequest, PatternResponse, RegisterHandlerRequest,
    RegisterHandlerResponse, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse, SpanStartRequest, SpanStartResponse, StdRegistryBridge, TracerRequest,
    TracerResponse, ValidatorRequest,
};
