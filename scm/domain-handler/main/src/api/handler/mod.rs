//! `Handler` theme — request/response execution port contracts.

pub mod errors;
pub mod in_process_handler_registry;
pub mod std_registry_bridge;
pub mod traits;
pub mod types;

pub use errors::HandlerError;
pub use in_process_handler_registry::InProcessHandlerRegistry;
pub use traits::{
    Handler, HandlerRegistry, IntoHandler, RegistryBridge, ServiceBridge, ServiceHandler, Validator,
};
pub use types::{
    BridgeRequest, BridgeResponse, DeregisterHandlerRequest, DeregisterHandlerResponse,
    EchoHandler, EmptinessRequest, EmptinessResponse, ExecutionRequest, HandlerContext,
    HandlerLookupRequest, HandlerLookupResponse, HealthCheckRequest, HealthCheckResponse,
    IdRequest, IdResponse, IntoHandlerRequest, IntoHandlerResponse, LenRequest, LenResponse,
    ListIdsRequest, ListIdsResponse, PatternRequest, PatternResponse, RegisterHandlerRequest,
    RegisterHandlerResponse, StdRegistryBridge, ValidatorRequest,
};
