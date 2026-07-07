//! # edge-domain-handler
//!
//! The `Handler` port contract — request/response execution units with registry and context.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

// Types that are part of trait signatures
pub use api::BridgeRequest;
pub use api::BridgeResponse;
pub use api::DeregisterHandlerRequest;
pub use api::DeregisterHandlerResponse;
pub use api::EchoHandler;
pub use api::EmptinessRequest;
pub use api::EmptinessResponse;
pub use api::ExecutionRequest;
pub use api::HandlerContext;
pub use api::HandlerError;
pub use api::HandlerLookupRequest;
pub use api::HandlerLookupResponse;
pub use api::HealthCheckRequest;
pub use api::HealthCheckResponse;
pub use api::IdRequest;
pub use api::IdResponse;
pub use api::InProcessHandlerRegistry;
pub use api::IntoHandlerRequest;
pub use api::IntoHandlerResponse;
pub use api::LenRequest;
pub use api::LenResponse;
pub use api::ListIdsRequest;
pub use api::ListIdsResponse;
pub use api::PatternRequest;
pub use api::PatternResponse;
pub use api::RegisterHandlerRequest;
pub use api::RegisterHandlerResponse;
pub use api::StdRegistryBridge;
pub use api::ValidatorRequest;

// Trait contracts
pub use api::Handler;
pub use api::HandlerRegistry;
pub use api::IntoHandler;
pub use api::RegistryBridge;
pub use api::ServiceBridge;
pub use api::ServiceHandler;
pub use api::Validator;

// SAF service identity constants
pub use saf::BRIDGE_CONTEXT;
pub use saf::HANDLER_REGISTRY_SVC;
pub use saf::HANDLER_REGISTRY_SVC_FACTORY;
pub use saf::HANDLER_SVC;
pub use saf::HANDLER_SVC_FACTORY;
pub use saf::INTO_HANDLER_SVC;
pub use saf::INTO_HANDLER_SVC_FACTORY;
pub use saf::MIN_SERVICE_NAME_LEN;
pub use saf::REGISTRY_BRIDGE_SVC;
pub use saf::REGISTRY_BRIDGE_SVC_FACTORY;
pub use saf::SERVICE_BRIDGE_SVC;
pub use saf::SERVICE_BRIDGE_SVC_FACTORY;
pub use saf::SERVICE_HANDLER_SVC_FACTORY;
pub use saf::VALIDATOR_SVC_FACTORY;
