use crate::api::types::SkillMetadata;

/// Response for [`Skill::metadata`](crate::api::traits::Skill::metadata).
pub struct SkillMetadataLookupResponse {
    /// Skill metadata including documentation and schemas.
    pub metadata: Box<SkillMetadata>,
}
