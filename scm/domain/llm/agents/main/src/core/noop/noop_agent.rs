//! No-op [`Agent`] implementation for testing the contract.

use std::sync::Arc;

use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderConfig, ProviderFactory, StdProviderFactory,
};

use crate::api::NoopAgent;
use crate::api::{Agent, AgentError, Skill};

#[async_trait::async_trait]
impl Agent for NoopAgent {
    fn id(&self) -> &str {
        Self::ID
    }

    fn name(&self) -> &str {
        Self::NAME
    }

    fn description(&self) -> &str {
        Self::DESCRIPTION
    }

    async fn execute_skill(&self, skill_name: &str, _input: String) -> Result<String, AgentError> {
        Err(AgentError::SkillNotFound(skill_name.to_string()))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        Arc::new(StdProviderFactory::provider(
            ProviderConfig::new("noop".to_string(), 0.0, 0),
            ModelInfo::default(),
            Arc::new(EchoProviderCompleter),
        ))
    }
}

impl NoopAgent {
    const ID: &'static str = "noop";
    const NAME: &'static str = "No-op Agent";
    const DESCRIPTION: &'static str = "Implements Agent trait; performs no work";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Role, ToolChoice};

    #[test]
    fn test_noop_agent_happy_id_returns_noop() {
        assert_eq!(NoopAgent.id(), "noop");
    }

    #[test]
    fn test_noop_agent_error_execute_skill_returns_skill_not_found() {
        let result =
            futures::executor::block_on(NoopAgent.execute_skill("any", "input".to_string()));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }

    #[test]
    fn test_noop_agent_edge_skills_returns_empty() {
        assert_eq!(NoopAgent.skills().len(), 0);
    }

    #[test]
    fn test_noop_agent_default_role_is_assistant() {
        assert_eq!(NoopAgent.supported_role(), Role::Assistant);
    }

    #[test]
    fn test_noop_agent_default_tool_choice_is_auto() {
        assert_eq!(NoopAgent.tool_choice(), ToolChoice::Auto);
    }
}
