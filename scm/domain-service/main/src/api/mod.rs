mod service;
pub use service::Service;
pub use service::ServiceRegistryTrait;
pub use service::ServiceRegistryBootstrap;

// Internal re-exports for crate use
pub(crate) use service::{ServiceError, NoopService, ServiceRegistry, StdServiceRegistryFactory};
