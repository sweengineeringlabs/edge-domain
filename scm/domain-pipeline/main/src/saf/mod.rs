//! Service Abstraction Framework — re-exports and service name constants.
//!
//! Provides trait re-exports and constants for service access.

pub mod pipeline_svc;
pub mod step_svc;
pub mod validator_svc;

/// Service name constant for pipeline service.
pub const PIPELINE_SVC: &str = "pipeline";

/// Service name constant for step service.
pub const STEP_SVC: &str = "step";

/// Service name constant for validator service.
pub const VALIDATOR_SVC: &str = "validator";
