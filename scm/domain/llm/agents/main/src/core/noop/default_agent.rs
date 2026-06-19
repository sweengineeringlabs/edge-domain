//! `DefaultAgent` — concrete [`Agent`] backed by a provider and a skill registry.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_security::SecurityContext;
use edge_llm_provider::Provider;

use crate::api::{Agent, AgentError, Skill};

/// Compact identity bundle — keeps `DefaultAgent` below the 5-field builder threshold.
struct DefaultAgentIdentity {
    id: String,
    name: String,
    description: String,
}

impl DefaultAgentIdentity {
    fn new(id: impl Into<String>, name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        }
    }
}

/// An [`Agent`] that routes `execute_skill` through a registered [`Skill`].
pub(crate) struct DefaultAgent {
    identity: DefaultAgentIdentity,
    provider: Arc<dyn Provider>,
    skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
}

impl DefaultAgent {
    /// Construct from identity fields, a provider, and a skill registry.
    pub(crate) fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        provider: Arc<dyn Provider>,
        skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
    ) -> Self {
        Self {
            identity: DefaultAgentIdentity::new(id, name, description),
            provider,
            skills,
        }
    }
}

#[async_trait]
impl Agent for DefaultAgent {
    fn id(&self) -> &str {
        &self.identity.id
    }
    fn name(&self) -> &str {
        &self.identity.name
    }
    fn description(&self) -> &str {
        &self.identity.description
    }
    fn provider(&self) -> Arc<dyn Provider> {
        Arc::clone(&self.provider)
    }
    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        self.skills.clone()
    }

    async fn execute_skill(&self, skill_name: &str, input: String) -> Result<String, AgentError> {
        let skill = self
            .skills
            .iter()
            .find(|s| s.name() == skill_name)
            .ok_or_else(|| AgentError::SkillNotFound(skill_name.to_string()))?;
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &commands);
        skill
            .execute(input, ctx)
            .await
            .map_err(|e| AgentError::ExecutionFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_llm_provider::{
        EchoProviderCompleter, ModelInfo, ProviderConfig, ProviderFactory, StdProviderFactory,
    };
    use futures::executor::block_on;

    fn noop_provider() -> Arc<dyn Provider> {
        Arc::new(StdProviderFactory::provider(
            ProviderConfig::new("noop".to_string(), 0.0, 0),
            ModelInfo::default(),
            Arc::new(EchoProviderCompleter),
        ))
    }

    /// @covers: new
    #[test]
    fn test_new_happy_fields_stored_and_retrievable() {
        let agent = DefaultAgent::new("a1", "Alpha", "does things", noop_provider(), vec![]);
        assert_eq!(Agent::id(&agent), "a1");
        assert_eq!(Agent::name(&agent), "Alpha");
        assert_eq!(Agent::description(&agent), "does things");
        assert!(Agent::skills(&agent).is_empty());
    }

    /// @covers: new
    #[test]
    fn test_new_error_provider_accessible_after_construction() {
        let provider = noop_provider();
        let agent = DefaultAgent::new("id", "name", "desc", Arc::clone(&provider), vec![]);
        assert_eq!(Agent::provider(&agent).name(), provider.name());
    }

    /// @covers: new
    #[test]
    fn test_new_edge_empty_id_stored_verbatim() {
        let agent = DefaultAgent::new("", "name", "desc", noop_provider(), vec![]);
        assert_eq!(Agent::id(&agent), "");
    }

    /// @covers: execute_skill
    #[test]
    fn test_execute_skill_error_unknown_skill_returns_not_found() {
        let agent = DefaultAgent::new("a", "A", "d", noop_provider(), vec![]);
        let result = block_on(agent.execute_skill("ghost", "x".to_string()));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }

    /// @covers: execute_skill
    #[test]
    fn test_execute_skill_edge_no_skills_returns_not_found() {
        let agent = DefaultAgent::new("a", "A", "d", noop_provider(), vec![]);
        let result = block_on(agent.execute_skill("any", String::new()));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }
}
