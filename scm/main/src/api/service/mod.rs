//! `Service` theme — named domain operation contracts.

pub mod error;
pub mod traits;
pub mod types;

pub use error::ServiceError;
pub use traits::Service;
pub use traits::ServiceRegistry;
