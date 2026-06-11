//! `Service` theme — named domain operation contracts.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::ServiceError;
pub use traits::Service;
pub use traits::ServiceRegistry;
