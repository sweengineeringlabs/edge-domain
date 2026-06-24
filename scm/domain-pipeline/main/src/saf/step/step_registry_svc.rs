//! Step registry service marker — re-exports the StepRegistry trait.

pub use crate::api::StepRegistry;

/// Service name constant for the step registry port.
pub const STEP_REGISTRY_SVC: &str = "step_registry";
