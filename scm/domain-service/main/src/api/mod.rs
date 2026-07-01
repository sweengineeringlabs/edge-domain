mod service;
pub use service::Service;
pub use service::ServiceRegistryTrait;
pub use service::ServiceRegistryBootstrap;

pub use service::ServiceError;
pub use service::{
    NoopService, ServiceRegistry, StdServiceRegistryFactory,
    NameRequest, NameResponse,
    RegisterServiceRequest, RegisterServiceResponse,
    ServiceRemovalRequest, ServiceRemovalResponse,
    ServiceLookupRequest, ServiceLookupResponse,
    ListNamesRequest, ListNamesResponse,
    LenRequest, LenResponse,
    EmptinessRequest, EmptinessResponse,
};
