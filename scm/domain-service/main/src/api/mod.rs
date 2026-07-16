mod service;
mod tests;

pub use service::Service;
pub use service::ServiceRegistry;

pub use service::ServiceError;
pub use service::{
    EmptinessRequest, EmptinessResponse, LenRequest, LenResponse, ListNamesRequest,
    ListNamesResponse, NameRequest, NameResponse, NoopRequest, NoopResponse, NoopService,
    RegisterServiceRequest, RegisterServiceResponse, ServiceLookupRequest, ServiceLookupResponse,
    ServiceRegistryStore, ServiceRemovalRequest, ServiceRemovalResponse, StdServiceRegistryFactory,
};
