//! [`StepCountResponse`] — wraps the number of steps in a pipeline.

/// Response carrying the number of steps registered in a pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepCountResponse {
    /// Number of steps in the pipeline.
    pub count: usize,
}
