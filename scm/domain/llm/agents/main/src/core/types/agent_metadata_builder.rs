//! Constructors and fluent setters for [`AgentMetadataBuilder`].

use crate::api::{AgentMetadata, AgentMetadataBuilder, SkillMetadata};

impl AgentMetadataBuilder {
    /// Create a new AgentMetadataBuilder.
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            description: None,
            version: None,
            skills: Vec::new(),
            patterns: Vec::new(),
        }
    }

    /// Set the agent ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the agent name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set the agent description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the agent version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Add a skill to the agent.
    pub fn skill(mut self, skill: SkillMetadata) -> Self {
        self.skills.push(skill);
        self
    }

    /// Set the skills for the agent.
    pub fn skills(mut self, skills: Vec<SkillMetadata>) -> Self {
        self.skills = skills;
        self
    }

    /// Add a pattern to the agent.
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.patterns.push(pattern.into());
        self
    }

    /// Set the patterns for the agent.
    pub fn patterns(mut self, patterns: Vec<String>) -> Self {
        self.patterns = patterns;
        self
    }

    /// Build the AgentMetadata.
    ///
    /// Unset string fields default to the empty string.
    pub fn build(self) -> AgentMetadata {
        AgentMetadata {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            version: self.version.unwrap_or_default(),
            skills: self.skills,
            patterns: self.patterns,
        }
    }
}

impl Default for AgentMetadataBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_has_no_id() {
        assert!(AgentMetadataBuilder::new().id.is_none());
    }

    /// @covers: build
    #[test]
    fn test_build_defaults_unset_strings_to_empty() {
        let metadata = AgentMetadataBuilder::new().build();
        assert_eq!(metadata.id, "");
    }

    /// @covers: id
    #[test]
    fn test_id_sets_id() {
        assert_eq!(
            AgentMetadataBuilder::new().id("a1").build().id,
            "a1".to_string()
        );
    }

    /// @covers: name
    #[test]
    fn test_name_sets_name() {
        assert_eq!(AgentMetadataBuilder::new().name("n").build().name, "n");
    }

    /// @covers: description
    #[test]
    fn test_description_sets_description() {
        assert_eq!(
            AgentMetadataBuilder::new()
                .description("d")
                .build()
                .description,
            "d"
        );
    }

    /// @covers: version
    #[test]
    fn test_version_sets_version() {
        assert_eq!(
            AgentMetadataBuilder::new().version("1.0").build().version,
            "1.0"
        );
    }

    /// @covers: skill
    #[test]
    fn test_skill_appends_one_skill() {
        let metadata = AgentMetadataBuilder::new()
            .skill(SkillMetadata::default())
            .build();
        assert_eq!(metadata.skills.len(), 1);
    }

    /// @covers: skills
    #[test]
    fn test_skills_replaces_skill_list() {
        let metadata = AgentMetadataBuilder::new()
            .skills(vec![SkillMetadata::default(), SkillMetadata::default()])
            .build();
        assert_eq!(metadata.skills.len(), 2);
    }

    /// @covers: pattern
    #[test]
    fn test_pattern_appends_one_pattern() {
        let metadata = AgentMetadataBuilder::new().pattern("react").build();
        assert_eq!(metadata.patterns, vec!["react".to_string()]);
    }

    /// @covers: patterns
    #[test]
    fn test_patterns_replaces_pattern_list() {
        let metadata = AgentMetadataBuilder::new()
            .patterns(vec!["react".to_string(), "cot".to_string()])
            .build();
        assert_eq!(metadata.patterns.len(), 2);
    }
}
