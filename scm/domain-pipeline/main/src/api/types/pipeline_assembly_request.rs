//! [`PipelineAssemblyRequest`] — request to assemble a pipeline from a definition.

use super::PipelineDefinition;

/// Request to assemble a pipeline by resolving each step name in `definition.steps`.
pub struct PipelineAssemblyRequest {
    /// The pipeline definition (config + ordered step names) to assemble.
    pub definition: Box<PipelineDefinition>,
}
