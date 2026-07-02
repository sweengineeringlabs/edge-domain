//! [`StepFailureResponse`] — wraps the assembled step-annotated error.

use crate::api::error::StepError;

/// Response carrying the assembled [`StepError`].
pub struct StepFailureResponse<E> {
    /// The step-annotated error.
    pub error: StepError<E>,
}
