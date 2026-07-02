//! [`PipelineConfigLookupRequest`] — zero-sized marker for [`Pipeline::config`](crate::Pipeline::config).

/// Marker request for querying a pipeline's configuration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PipelineConfigLookupRequest;
