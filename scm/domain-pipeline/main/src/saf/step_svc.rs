//! Step service — exposes the Step trait for composition.
//!
//! The Step trait is re-exported here to provide a unified service interface.
//! Steps are typically created by domain logic or composite pipelines.

// Step trait is accessed through saf::step_svc for service consistency,
// even though there are no factory functions for Step creation.
