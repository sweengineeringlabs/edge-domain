mod service;

pub use service::{
    StdServiceRegistryFactory,
    NoopService,
    Service,
    ServiceError,
    ServiceRegistry,
    ServiceRegistryBootstrap,
    ServiceRegistryImpl,
    SERVICE_REGISTRY_FACTORY_SVC,
    SERVICE_REGISTRY_SVC,
    SERVICE_SVC,
};
