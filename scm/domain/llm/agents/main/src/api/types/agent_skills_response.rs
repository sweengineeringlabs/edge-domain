use std::sync::Arc;

use crate::api::Skill;

/// Response for [`Agent::skills`](crate::api::traits::Agent::skills).
pub struct AgentSkillsResponse {
    /// All available skills.
    pub skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
}
