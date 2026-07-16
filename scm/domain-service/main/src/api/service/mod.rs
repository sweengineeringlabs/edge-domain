//! `Service` theme — named domain operations with registry.

pub mod dto;
pub mod errors;
pub mod noop_service;
pub mod service_registry_store;
pub mod traits;
pub mod vo;

pub use dto::{
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NameRequest, NameResponse, NoopRequest, NoopResponse,
    RegisterServiceRequest, RegisterServiceResponse, Request, Response, ServiceLookupRequest,
    ServiceLookupResponse, ServiceRemovalRequest, ServiceRemovalResponse,
};
pub use errors::ServiceError;
pub use noop_service::NoopService;
pub use service_registry_store::ServiceRegistryStore;
pub use traits::{Service, ServiceRegistry};
pub use vo::StdServiceRegistryFactory;
