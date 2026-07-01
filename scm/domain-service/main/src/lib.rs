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
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NameRequest, NameResponse, NoopService, RegisterServiceRequest,
    RegisterServiceResponse, Service, ServiceError, ServiceLookupRequest, ServiceLookupResponse,
    ServiceRegistry, ServiceRegistryStore, ServiceRemovalRequest, ServiceRemovalResponse,
    StdServiceRegistryFactory,
};
pub use saf::{SERVICE_REGISTRY_SVC, SERVICE_SVC};
