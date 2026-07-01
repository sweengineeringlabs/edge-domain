mod service;
pub use service::Service;
pub use service::ServiceRegistry;
pub use service::ServiceRegistryBootstrap;

pub use service::ServiceError;
pub use service::{
    NoopService, ServiceRegistryStore, StdServiceRegistryFactory,
    NameRequest, NameResponse,
    RegisterServiceRequest, RegisterServiceResponse,
    ServiceRemovalRequest, ServiceRemovalResponse,
    ServiceLookupRequest, ServiceLookupResponse,
    ListNamesRequest, ListNamesResponse,
    LenRequest, LenResponse,
    EmptinessRequest, EmptinessResponse,
};
