use crate::api::types::SkillMetadataBuilder;

/// Response for [`AgentManager::skill_metadata_builder`](crate::api::traits::AgentManager::skill_metadata_builder).
pub struct SkillMetadataBuilderResponse {
    /// A builder for constructing [`SkillMetadata`](crate::api::types::SkillMetadata).
    pub builder: Box<SkillMetadataBuilder>,
}
