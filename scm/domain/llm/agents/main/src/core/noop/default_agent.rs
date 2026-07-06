//! `DefaultAgent` — concrete [`Agent`] backed by a provider and a skill registry.

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_handler::ExecutionRequest;
use edge_llm_provider::Provider;

use crate::api::{Agent, AgentError, Skill};
use crate::api::{
    AgentDescriptionRequest, AgentDescriptionResponse, AgentIdRequest, AgentIdResponse,
    AgentNameRequest, AgentNameResponse, AgentProviderRequest, AgentProviderResponse,
    AgentSkillsRequest, AgentSkillsResponse, SkillExecutionRequest, SkillExecutionResponse,
    SkillNameRequest,
};

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
    fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
        Ok(AgentIdResponse {
            id: self.identity.id.clone(),
        })
    }
    fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
        Ok(AgentNameResponse {
            name: self.identity.name.clone(),
        })
    }
    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError> {
        Ok(AgentDescriptionResponse {
            description: self.identity.description.clone(),
        })
    }
    fn provider(&self, _req: AgentProviderRequest) -> Result<AgentProviderResponse, AgentError> {
        Ok(AgentProviderResponse {
            provider: Arc::clone(&self.provider),
        })
    }
    fn skills(&self, _req: AgentSkillsRequest) -> Result<AgentSkillsResponse, AgentError> {
        Ok(AgentSkillsResponse {
            skills: self.skills.clone(),
        })
    }

    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError> {
        let skill = self
            .skills
            .iter()
            .find(|s| {
                s.name(SkillNameRequest)
                    .map(|r| r.name == req.skill_name)
                    .unwrap_or(false)
            })
            .ok_or_else(|| AgentError::SkillNotFound(req.skill_name.to_string()))?;
        skill
            .execute(ExecutionRequest {
                req: req.input,
                ctx: &req.ctx,
            })
            .await
            .map(|output| SkillExecutionResponse { output })
            .map_err(|e| AgentError::ExecutionFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_observer::StdObserveFactory;
    use edge_llm_provider::{
        EchoProviderCompleter, ModelInfo, ProviderBootstrap, ProviderConfig, StdProviderFactory,
    };
    use futures::executor::block_on;

    fn noop_provider() -> Arc<dyn Provider> {
        StdProviderFactory::provider(
            ProviderConfig::new("noop".to_string(), 0.0, 0),
            Box::<ModelInfo>::default(),
            Arc::new(EchoProviderCompleter),
            StdObserveFactory::noop_arc_observe_context(),
        )
    }

    /// @covers: new
    #[test]
    fn test_new_happy_fields_stored_and_retrievable() {
        let agent = DefaultAgent::new("a1", "Alpha", "does things", noop_provider(), vec![]);
        assert_eq!(Agent::id(&agent, AgentIdRequest).unwrap().id, "a1");
        assert_eq!(Agent::name(&agent, AgentNameRequest).unwrap().name, "Alpha");
        assert_eq!(
            Agent::description(&agent, AgentDescriptionRequest)
                .unwrap()
                .description,
            "does things"
        );
        assert!(Agent::skills(&agent, AgentSkillsRequest)
            .unwrap()
            .skills
            .is_empty());
    }

    /// @covers: new
    #[test]
    fn test_new_error_provider_accessible_after_construction() {
        use edge_llm_provider::ProviderNameRequest;
        let provider = noop_provider();
        let agent = DefaultAgent::new("id", "name", "desc", Arc::clone(&provider), vec![]);
        assert_eq!(
            Agent::provider(&agent, AgentProviderRequest)
                .unwrap()
                .provider
                .name(ProviderNameRequest)
                .unwrap()
                .name,
            provider.name(ProviderNameRequest).unwrap().name
        );
    }

    /// @covers: new
    #[test]
    fn test_new_edge_empty_id_stored_verbatim() {
        let agent = DefaultAgent::new("", "name", "desc", noop_provider(), vec![]);
        assert_eq!(Agent::id(&agent, AgentIdRequest).unwrap().id, "");
    }

    /// @covers: execute_skill
    #[test]
    fn test_execute_skill_error_unknown_skill_returns_not_found() {
        use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
        use edge_domain_handler::HandlerContext;
        use edge_domain_observer::StdObserveFactory;
        use edge_security_runtime::SecurityContext;
        let agent = DefaultAgent::new("a", "A", "d", noop_provider(), vec![]);
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let result = block_on(agent.execute_skill(SkillExecutionRequest {
            skill_name: "ghost",
            input: "x".to_string(),
            ctx,
        }));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }

    /// @covers: execute_skill
    #[test]
    fn test_execute_skill_edge_no_skills_returns_not_found() {
        use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
        use edge_domain_handler::HandlerContext;
        use edge_domain_observer::StdObserveFactory;
        use edge_security_runtime::SecurityContext;
        let agent = DefaultAgent::new("a", "A", "d", noop_provider(), vec![]);
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let result = block_on(agent.execute_skill(SkillExecutionRequest {
            skill_name: "any",
            input: String::new(),
            ctx,
        }));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }
}
