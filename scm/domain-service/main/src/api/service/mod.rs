//! `Service` theme — named domain operations with registry.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ServiceError;
pub use traits::{Service, ServiceRegistry};
pub use types::{
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NameRequest, NameResponse, NoopService, RegisterServiceRequest,
    RegisterServiceResponse, ServiceLookupRequest, ServiceLookupResponse, ServiceRegistryStore,
    ServiceRemovalRequest, ServiceRemovalResponse, StdServiceRegistryFactory,
};
