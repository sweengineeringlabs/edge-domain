//! `Service` theme — named domain operations with registry.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ServiceError;
pub use traits::{Service, ServiceRegistry as ServiceRegistryTrait, ServiceRegistryBootstrap};
pub use types::{
    StdServiceRegistryFactory, NoopService, ServiceRegistry,
    NameRequest, NameResponse,
    RegisterServiceRequest, RegisterServiceResponse,
    ServiceRemovalRequest, ServiceRemovalResponse,
    ServiceLookupRequest, ServiceLookupResponse,
    ListNamesRequest, ListNamesResponse,
    LenRequest, LenResponse,
    EmptinessRequest, EmptinessResponse,
};
