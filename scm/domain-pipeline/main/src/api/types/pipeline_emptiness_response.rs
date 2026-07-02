//! [`PipelineEmptinessResponse`] — wraps whether a pipeline has no steps.

/// Response carrying whether a pipeline has zero steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipelineEmptinessResponse {
    /// `true` when the pipeline has no steps.
    pub empty: bool,
}
