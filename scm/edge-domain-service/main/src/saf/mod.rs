//! SAF — service facade.

mod service;

pub use crate::api::service::Service;
pub use crate::api::service::ServiceError;
pub use crate::api::service::ServiceRegistry;
pub use crate::api::service::ServiceRegistryFactory;
pub use crate::api::service::ServiceRegistryTrait as ServiceRegistryImpl;
