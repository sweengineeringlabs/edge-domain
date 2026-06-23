//! Step service — exposes the Step trait for composition.
//!
//! The Step trait is re-exported here to provide a unified service interface.
//! Steps are typically created by domain logic or composite pipelines.

/// Re-export of the Step trait for service interface.
pub use crate::api::Step;
