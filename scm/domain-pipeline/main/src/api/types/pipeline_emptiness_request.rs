//! [`PipelineEmptinessRequest`] — zero-sized marker for [`Pipeline::is_empty`](crate::Pipeline::is_empty).

/// Marker request for checking whether a pipeline has no steps.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PipelineEmptinessRequest;
