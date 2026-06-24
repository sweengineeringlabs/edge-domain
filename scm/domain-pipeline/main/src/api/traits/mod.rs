//! Trait contracts for the pipeline domain.

pub mod validator;
pub mod step_registry;

pub use super::step::Step;
pub use validator::Validator;
pub use step_registry::StepRegistry;
