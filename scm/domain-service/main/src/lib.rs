//! # edge-domain-service
//!
//! The `Service` port contract — named domain operations with ServiceRegistry.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::ServiceRegistry as ServiceRegistryImpl;
pub use saf::Service;
pub use saf::ServiceError;
pub use saf::ServiceRegistry;
pub use saf::ServiceRegistryBootstrap;
pub use saf::SERVICE_REGISTRY_FACTORY_SVC;
pub use saf::SERVICE_REGISTRY_SVC;
pub use saf::SERVICE_SVC;
