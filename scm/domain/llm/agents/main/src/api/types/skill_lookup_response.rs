use std::sync::Arc;

use crate::api::Skill;

/// Response for [`Agent::skill`](crate::api::traits::Agent::skill).
pub struct SkillLookupResponse {
    /// The resolved skill.
    pub skill: Arc<dyn Skill<Request = String, Response = String>>,
}
