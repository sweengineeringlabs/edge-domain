//! Constructors and fluent setters for [`SkillMetadataBuilder`].

use crate::api::{SkillMetadata, SkillMetadataBuilder};

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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_has_no_name() {
        assert!(SkillMetadataBuilder::new().name.is_none());
    }

    /// @covers: build
    #[test]
    fn test_build_defaults_unset_strings_to_empty() {
        let metadata = SkillMetadataBuilder::new().build();
        assert_eq!(metadata.name, "");
    }

    /// @covers: name
    #[test]
    fn test_name_sets_name() {
        assert_eq!(SkillMetadataBuilder::new().name("n").build().name, "n");
    }

    /// @covers: description
    #[test]
    fn test_description_sets_description() {
        assert_eq!(
            SkillMetadataBuilder::new()
                .description("d")
                .build()
                .description,
            "d"
        );
    }

    /// @covers: input_schema
    #[test]
    fn test_input_schema_sets_schema() {
        let metadata = SkillMetadataBuilder::new().input_schema("{}").build();
        assert_eq!(metadata.input_schema, Some("{}".to_string()));
    }

    /// @covers: output_schema
    #[test]
    fn test_output_schema_sets_schema() {
        let metadata = SkillMetadataBuilder::new().output_schema("{}").build();
        assert_eq!(metadata.output_schema, Some("{}".to_string()));
    }

    /// @covers: async_execution
    #[test]
    fn test_async_execution_sets_flag() {
        assert!(
            SkillMetadataBuilder::new()
                .async_execution(true)
                .build()
                .async_execution
        );
    }

    /// @covers: long_running
    #[test]
    fn test_long_running_sets_flag() {
        assert!(
            SkillMetadataBuilder::new()
                .long_running(true)
                .build()
                .long_running
        );
    }
}
