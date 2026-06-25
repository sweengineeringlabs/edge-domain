use swe_edge_configbuilder::ConfigSection;

use crate::api::PipelineConfig;

const PIPELINE_SECTION_NAME: &str = "pipeline";

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: true,
        }
    }
}

impl ConfigSection for PipelineConfig {
    fn section_name() -> &'static str {
        PIPELINE_SECTION_NAME
    }
}
