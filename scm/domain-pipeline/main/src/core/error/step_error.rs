//! [`Display`] and [`Error`] implementations for [`StepError<E>`].

use std::fmt;

use crate::api::StepError;

impl<E: fmt::Display> fmt::Display for StepError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "step '{}' failed: {}", self.step_name, self.cause)
    }
}

impl<E: std::error::Error + 'static> std::error::Error for StepError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}
