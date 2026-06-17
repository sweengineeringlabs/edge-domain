//! Builder for SkillMetadata with fluent API.

use crate::api::types::SkillMetadata;

/// Builder for SkillMetadata with fluent setter pattern.
#[derive(Debug, Clone)]
pub struct SkillMetadataBuilder {
    name: Option<String>,
    description: Option<String>,
    input_schema: Option<String>,
    output_schema: Option<String>,
    async_execution: bool,
    long_running: bool,
}

impl SkillMetadataBuilder {
    /// Create a new SkillMetadataBuilder.
    pub fn new() -> Self {
        Self {
            name: None,
            description: None,
            input_schema: None,
            output_schema: None,
            async_execution: false,
            long_running: false,
        }
    }

    /// Set the skill name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the skill description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the input schema.
    pub fn input_schema(mut self, schema: impl Into<String>) -> Self {
        self.input_schema = Some(schema.into());
        self
    }

    /// Set the output schema.
    pub fn output_schema(mut self, schema: impl Into<String>) -> Self {
        self.output_schema = Some(schema.into());
        self
    }

    /// Set whether the skill executes asynchronously.
    pub fn async_execution(mut self, async_execution: bool) -> Self {
        self.async_execution = async_execution;
        self
    }

    /// Set whether the skill is long-running.
    pub fn long_running(mut self, long_running: bool) -> Self {
        self.long_running = long_running;
        self
    }

    /// Build the SkillMetadata.
    ///
    /// Unset string fields default to the empty string.
    pub fn build(self) -> SkillMetadata {
        SkillMetadata {
            name: self.name.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            input_schema: self.input_schema,
            output_schema: self.output_schema,
            async_execution: self.async_execution,
            long_running: self.long_running,
        }
    }
}

impl Default for SkillMetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}
