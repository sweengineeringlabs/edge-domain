use crate::api::{NoopService, ServiceError};

/// SAF service identifier for the `Service` trait.
pub const SERVICE_SVC: &str = "service";

pub use crate::api::Service;
