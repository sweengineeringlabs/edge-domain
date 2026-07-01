//! # edge-domain-service
//!
//! The `Service` port contract — named domain operations with ServiceRegistry.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::{
    NoopService, Service, ServiceError, ServiceRegistry, ServiceRegistryBootstrap,
    ServiceRegistryTrait, StdServiceRegistryFactory,
    NameRequest, NameResponse,
    RegisterServiceRequest, RegisterServiceResponse,
    ServiceRemovalRequest, ServiceRemovalResponse,
    ServiceLookupRequest, ServiceLookupResponse,
    ListNamesRequest, ListNamesResponse,
    LenRequest, LenResponse,
    EmptinessRequest, EmptinessResponse,
};
pub use saf::{SERVICE_REGISTRY_SVC, SERVICE_SVC};
