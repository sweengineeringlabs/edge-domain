//! [`BuilderValidationRequest`] — request wrapping a pipeline builder to validate.

use crate::api::PipelineBuilder;

/// Request to validate the configuration embedded in a [`PipelineBuilder`].
pub struct BuilderValidationRequest<'a, Ctx, E> {
    /// The builder whose embedded configuration should be validated.
    pub builder: &'a PipelineBuilder<Ctx, E>,
}
