//! `Service` module — named domain operation contracts.

#[allow(clippy::module_inception)]
pub mod service;
pub mod service_error;

pub use service::Service;
pub use service_error::ServiceError;
