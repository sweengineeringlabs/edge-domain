//! `Handler` theme — request/response execution port contracts.

pub mod errors;
pub mod in_process_handler_registry;
pub mod service_handler;
pub mod std_registry_bridge;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use service_handler::ServiceHandler;
pub use traits::{
    Handler, HandlerBootstrap, HandlerProvider, HandlerRegistry, IntoHandler, RegistryBridge,
    ServiceBridge, Validator,
};
pub use types::{
    BootstrapNameRequest, BootstrapNameResponse, BridgeRequest, BridgeResponse,
    DeregisterHandlerRequest, DeregisterHandlerResponse, EchoHandler, EmptinessRequest,
    EmptinessResponse, ExecutionRequest, HandlerBuildResponse, HandlerContext,
    HandlerLookupRequest, HandlerLookupResponse, HealthCheckRequest, HealthCheckResponse,
    IdRequest, IdResponse, IntoHandlerRequest, IntoHandlerResponse, LenRequest, LenResponse,
    ListIdsRequest, ListIdsResponse, NoopHandlerFactory, PatternRequest, PatternResponse,
    RegisterHandlerRequest, RegisterHandlerResponse, StdRegistryBridge, ValidatorRequest,
};
