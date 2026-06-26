//! [`StepError<E>`] — engine-owned wrapper produced when a step returns `Err`.

/// Engine-owned error wrapper produced when a step returns `Err(cause)`.
///
/// The engine adds `step_name` context; the consumer's typed error `E` is preserved
/// in `cause`. Callers receive this inside [`PipelineError::StepFailed`].
#[derive(Debug)]
pub struct StepError<E> {
    /// Name of the step that failed (from [`Step::name`](crate::Step::name)).
    pub step_name: String,
    /// The typed error returned by the step.
    pub cause: E,
}
