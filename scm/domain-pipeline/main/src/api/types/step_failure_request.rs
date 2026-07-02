//! [`StepFailureRequest`] — request to wrap a cause in a step-annotated error.

/// Request to wrap `cause` in a [`StepError`](crate::StepError) annotated with `step_name`.
///
/// Shared by [`Step::fail_with`](crate::Step::fail_with) and
/// [`StepRegistry::step_error_for`](crate::StepRegistry::step_error_for).
pub struct StepFailureRequest<E> {
    /// Name of the step the error is attributed to.
    pub step_name: String,
    /// The typed cause to wrap.
    pub cause: E,
}
