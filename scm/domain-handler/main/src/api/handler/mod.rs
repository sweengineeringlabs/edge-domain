//! `Handler` theme — request/response execution port contracts.

pub mod command_bus_adapter;
pub mod dto;
mod echo_handler;
pub mod errors;
pub mod in_process_handler_registry;
mod observability_bridge;
pub mod observer_context_adapter;
mod service_handler;
pub mod std_registry_bridge;
pub mod traits;
pub mod vo;

pub use command_bus_adapter::CommandBusAdapter;
pub use dto::{
    BridgeRequest, BridgeResponse, CommandDispatchRequest, CommandExecutionRequest,
    CommandNameRequest, CommandNameResponse, CounterLookupRequest, CounterLookupResponse,
    DeregisterHandlerRequest, DeregisterHandlerResponse, DrainRequest, DrainResponse,
    EmptinessRequest, EmptinessResponse, ExecutionRequest, GaugeLookupRequest,
    GaugeLookupResponse, GaugeSetRequest, GaugeSetResponse, HandlerLookupRequest,
    HandlerLookupResponse, HealthCheckRequest, HealthCheckResponse, HistogramLookupRequest,
    HistogramLookupResponse, HistogramRecordRequest, HistogramRecordResponse, IdRequest,
    IdResponse, IncrementRequest, IncrementResponse, IntoHandlerRequest, IntoHandlerResponse,
    LenRequest, LenResponse, ListIdsRequest, ListIdsResponse, ListNamesRequest, ListNamesResponse,
    LogEmitRequest, LogEmitResponse, MetricsRequest, MetricsResponse, PatternRequest,
    PatternResponse, RegisterHandlerRequest, RegisterHandlerResponse, ServiceLookupRequest,
    ServiceLookupResponse, SpanAnnotationRequest, SpanAnnotationResponse, SpanFinishRequest,
    SpanFinishResponse, SpanStartRequest, SpanStartResponse, TracerRequest, TracerResponse,
    ValidatorRequest,
};
pub use echo_handler::EchoHandler;
pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use observer_context_adapter::ObserverContextAdapter;
pub use std_registry_bridge::StdRegistryBridge;
pub use traits::{
    Command, CommandBus, Counter, Gauge, Handler, HandlerRegistry, HandlerTracer, Histogram,
    IntoHandler, LogDrain, MetricRegistry, ObserverContext, RegistryBridge, SecurityPrincipal,
    Service, ServiceBridge, ServiceHandler, ServiceRegistry, Span, Validator,
};
pub use vo::HandlerContext;
