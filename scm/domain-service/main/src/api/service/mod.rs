//! `Service` theme — named domain operations with registry.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ServiceError;
pub use traits::{Service, ServiceRegistry, ServiceRegistryBootstrap};
pub use types::{
    StdServiceRegistryFactory, NoopService, ServiceRegistryStore,
    NameRequest, NameResponse,
    RegisterServiceRequest, RegisterServiceResponse,
    ServiceRemovalRequest, ServiceRemovalResponse,
    ServiceLookupRequest, ServiceLookupResponse,
    ListNamesRequest, ListNamesResponse,
    LenRequest, LenResponse,
    EmptinessRequest, EmptinessResponse,
};
