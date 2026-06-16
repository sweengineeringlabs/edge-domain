//! No-op Agent implementation for testing the contract.

use crate::api::{Agent, AgentError, Skill};
use std::sync::Arc;

/// A no-op agent that implements the Agent trait but performs no work.
/// Used for testing the contract; real implementations live in plugins.
pub(crate) struct NoopAgent;

#[async_trait::async_trait]
impl Agent for NoopAgent {
    fn id(&self) -> &str {
        "noop"
    }

    fn name(&self) -> &str {
        "No-op Agent"
    }

    fn description(&self) -> &str {
        "Implements Agent trait; performs no work"
    }

    async fn execute_skill(
        &self,
        skill_name: &str,
        _input: String,
    ) -> Result<String, AgentError> {
        Err(AgentError::SkillNotFound(skill_name.to_string()))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_agent_happy_id_returns_noop() {
        assert_eq!(NoopAgent.id(), "noop");
    }

    #[test]
    fn test_noop_agent_error_execute_skill_returns_skill_not_found() {
        let result = futures::executor::block_on(NoopAgent.execute_skill("any", "input".to_string()));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }

    #[test]
    fn test_noop_agent_edge_skills_returns_empty() {
        assert_eq!(NoopAgent.skills().len(), 0);
    }
}
