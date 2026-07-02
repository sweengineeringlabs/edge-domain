//! Builder for SkillMetadata with fluent API.

/// Builder for SkillMetadata with fluent setter pattern.
#[derive(Debug, Clone)]
pub struct SkillMetadataBuilder {
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) input_schema: Option<String>,
    pub(crate) output_schema: Option<String>,
    pub(crate) async_execution: bool,
    pub(crate) long_running: bool,
}
