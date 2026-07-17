mod service;
mod tests;

pub use service::Service;
pub use service::ServiceRegistry;

pub use service::ServiceError;
pub use service::{
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NameRequest, NameResponse, NoopService, RegisterServiceRequest,
    RegisterServiceResponse, Request, Response, ServiceLookupRequest, ServiceLookupResponse,
    ServiceRegistryStore, ServiceRemovalRequest, ServiceRemovalResponse,
    StdServiceRegistryFactory,
};
