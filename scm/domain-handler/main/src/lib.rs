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
pub use api::StdRegistryBridge;
pub use api::EchoHandler;
pub use api::HandlerContext;
pub use api::HandlerError;
pub use api::InProcessHandlerRegistry;
pub use api::NoopHandlerFactory;

// Trait contracts
pub use api::Handler;
pub use api::HandlerBootstrap;
pub use api::HandlerProvider;
pub use api::HandlerRegistry;
pub use api::IntoHandler;
pub use api::RegistryBridge;
pub use api::ServiceBridge;
pub use api::ServiceHandler;
pub use api::Validator;

// SAF service identity constants
pub use saf::BRIDGE_CONTEXT;
pub use saf::HANDLER_BOOTSTRAP_SVC;
pub use saf::HANDLER_BOOTSTRAP_SVC_FACTORY;
pub use saf::HANDLER_PROVIDER_SVC;
pub use saf::HANDLER_PROVIDER_SVC_FACTORY;
pub use saf::HANDLER_REGISTRY_SVC;
pub use saf::HANDLER_REGISTRY_SVC_FACTORY;
pub use saf::HANDLER_SVC;
pub use saf::HANDLER_SVC_FACTORY;
pub use saf::INTO_HANDLER_SVC;
pub use saf::INTO_HANDLER_SVC_FACTORY;
pub use saf::MIN_SERVICE_NAME_LEN;
pub use saf::SERVICE_BRIDGE_SVC;
pub use saf::SERVICE_BRIDGE_SVC_FACTORY;
pub use saf::REGISTRY_BRIDGE_SVC;
pub use saf::REGISTRY_BRIDGE_SVC_FACTORY;
pub use saf::SERVICE_HANDLER_SVC_FACTORY;
pub use saf::VALIDATOR_SVC_FACTORY;
