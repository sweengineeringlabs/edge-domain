//! [`Display`] and [`Error`] implementations for [`PipelineError<E>`].

use std::fmt;

use crate::api::PipelineError;

impl<E: fmt::Display + fmt::Debug> fmt::Display for PipelineError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineError::StepFailed(e) => write!(f, "step failed: {e}"),
            PipelineError::StepTimeout { step_name } => {
                write!(f, "step '{step_name}' timed out")
            }
            PipelineError::ConfigError(msg) => write!(f, "configuration error: {msg}"),
            PipelineError::UnknownStep(name) => write!(f, "unknown step: {name}"),
        }
    }
}

impl<E: std::error::Error + Send + 'static> std::error::Error for PipelineError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let PipelineError::StepFailed(e) = self {
            Some(&e.cause)
        } else {
            None
        }
    }
}
