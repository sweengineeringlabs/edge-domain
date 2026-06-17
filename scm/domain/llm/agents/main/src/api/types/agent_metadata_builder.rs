//! Builder for AgentMetadata with fluent API.

use crate::api::types::{AgentMetadata, SkillMetadata};

/// Builder for AgentMetadata with fluent setter pattern.
#[derive(Debug, Clone)]
pub struct AgentMetadataBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    skills: Vec<SkillMetadata>,
    patterns: Vec<String>,
}

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
