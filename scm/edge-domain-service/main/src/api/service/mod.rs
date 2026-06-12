//! `Service` theme — named domain operations with registry.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ServiceError;
pub use traits::{Service, ServiceRegistry as ServiceRegistryTrait, ServiceRegistryFactory};
pub use types::ServiceRegistry;
