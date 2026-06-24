//! [`PipelineDefinition`] — TOML manifest declaring step composition and execution config.

use swe_edge_configbuilder::ConfigSection;

use crate::api::types::PipelineConfig;

const PIPELINE_DEFINITION_SECTION: &str = "pipeline";

/// TOML manifest for a pipeline: execution configuration plus an ordered list of step names.
///
/// Load from TOML via [`ConfigSection::load`], then pass to
/// [`StepRegistry::build_pipeline`](crate::StepRegistry::build_pipeline) to assemble
/// a concrete pipeline.
///
/// ## TOML section `[pipeline]`
/// ```toml
/// [pipeline]
/// abort_on_error        = true
/// timeout_per_step_ms   = 5000
/// emit_lifecycle_events = false
/// steps = ["validate-input", "enrich-data", "publish-event"]
/// ```
#[derive(Clone, Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct PipelineDefinition {
    /// Execution configuration (timeout, error handling, lifecycle events).
    #[serde(flatten)]
    pub config: PipelineConfig,

    /// Ordered step names. Each name must be registered in the [`StepRegistry`](crate::StepRegistry)
    /// before calling `build_pipeline`; an unrecognised name yields
    /// [`PipelineError::UnknownStep`](crate::PipelineError::UnknownStep).
    pub steps: Vec<String>,
}

impl ConfigSection for PipelineDefinition {
    fn section_name() -> &'static str {
        PIPELINE_DEFINITION_SECTION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_definition_default_happy() {
        let def = PipelineDefinition::default();
        assert!(def.steps.is_empty());
        assert!(def.config.abort_on_error);
        assert!(def.config.timeout_per_step.is_none());
    }

    #[test]
    fn test_pipeline_definition_section_name_happy() {
        assert_eq!(PipelineDefinition::section_name(), "pipeline");
    }
}
