//! [`StepCountRequest`] — zero-sized marker for [`Pipeline::step_count`](crate::Pipeline::step_count).

/// Marker request for querying the number of steps in a pipeline.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StepCountRequest;
