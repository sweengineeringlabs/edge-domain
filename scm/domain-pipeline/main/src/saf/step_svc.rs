//! Step service — marker for step composition operations.
//!
//! The Step trait is not re-exported here as it's already available from the api module.
//! This module exists to provide a consistent service facade pattern across the crate.

/// Service marker constant for step operations.
pub const STEP_FACTORY: &str = "step_factory";
