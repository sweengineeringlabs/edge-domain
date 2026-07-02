//! [`StepRegistrationRequest`] — request to register a step under a name.

use std::sync::Arc;

use crate::api::Step;

/// Request to register a shared step instance under `name`.
///
/// Replaces any prior registration for the same name.
pub struct StepRegistrationRequest<Ctx, E> {
    /// Name the step is registered under.
    pub name: String,
    /// The shared step instance.
    pub step: Arc<dyn Step<Ctx = Ctx, ExecutionError = E>>,
}
