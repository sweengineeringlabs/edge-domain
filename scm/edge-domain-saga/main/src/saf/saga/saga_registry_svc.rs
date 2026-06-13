/// Service name constant for the saga registry port.
pub const SAGA_REGISTRY_SVC: &str = "saga_registry";

pub use crate::api::saga::InMemorySagaRegistry;
pub use crate::api::saga::SagaError;
pub use crate::api::saga::SagaRegistry;
