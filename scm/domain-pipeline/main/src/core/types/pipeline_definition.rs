use swe_edge_configbuilder::ConfigSection;

use crate::api::PipelineDefinition;

const PIPELINE_DEFINITION_SECTION: &str = "pipeline";

impl ConfigSection for PipelineDefinition {
    fn section_name() -> &'static str {
        PIPELINE_DEFINITION_SECTION
    }
}
