//! Builder for AgentMetadata with fluent API.

use crate::api::types::SkillMetadata;

/// Builder for AgentMetadata with fluent setter pattern.
#[derive(Debug, Clone)]
pub struct AgentMetadataBuilder {
    pub(crate) id: Option<String>,
    pub(crate) name: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) version: Option<String>,
    pub(crate) skills: Vec<SkillMetadata>,
    pub(crate) patterns: Vec<String>,
}
