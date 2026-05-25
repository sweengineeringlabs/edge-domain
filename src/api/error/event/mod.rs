//! Event-related error types.

pub mod error;
pub mod store_error;

pub use error::EventError;
pub use store_error::EventStoreError;
