//! No-op [`Agent`] implementation for testing the contract.

use std::sync::Arc;

use edge_domain_observer::StdObserveFactory;
use edge_llm_provider::{EchoProviderCompleter, ModelInfo, ProviderConfig, StdProvider};

use crate::api::NoopAgent;
use crate::api::{Agent, AgentError};
use crate::api::{
    AgentDescriptionRequest, AgentDescriptionResponse, AgentIdRequest, AgentIdResponse,
    AgentNameRequest, AgentNameResponse, AgentProviderRequest, AgentProviderResponse,
    AgentSkillsRequest, AgentSkillsResponse, SkillExecutionRequest, SkillExecutionResponse,
};

#[async_trait::async_trait]
impl Agent for NoopAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
        Ok(AgentIdResponse {
            id: Self::ID.to_string(),
        })
    }

    fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
        Ok(AgentNameResponse {
            name: Self::NAME.to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError> {
        Ok(AgentDescriptionResponse {
            description: Self::DESCRIPTION.to_string(),
        })
    }

    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError> {
        Err(AgentError::SkillNotFound(req.skill_name.to_string()))
    }

    fn skills(&self, _req: AgentSkillsRequest) -> Result<AgentSkillsResponse, AgentError> {
        Ok(AgentSkillsResponse { skills: vec![] })
    }

    fn provider(&self, _req: AgentProviderRequest) -> Result<AgentProviderResponse, AgentError> {
        Ok(AgentProviderResponse {
            provider: Arc::new(StdProvider::new(
                ProviderConfig::new("noop".to_string(), 0.0, 0),
                ModelInfo::default(),
                Arc::new(EchoProviderCompleter),
                StdObserveFactory::noop_arc_observe_context(),
            )),
        })
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
    use crate::api::{SupportedRoleRequest, ToolChoicePreferenceRequest};

    #[test]
    fn test_noop_agent_happy_id_returns_noop() {
        assert_eq!(NoopAgent.id(AgentIdRequest).unwrap().id, "noop");
    }

    #[test]
    fn test_noop_agent_error_execute_skill_returns_skill_not_found() {
        use edge_domain_command::DirectCommandBus;
        use edge_domain_handler::HandlerContext;
        use edge_domain_observer::StdObserveFactory;
        use edge_security_runtime::SecurityContext;
        let security = SecurityContext::unauthenticated();
        let commands = DirectCommandBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let result = futures::executor::block_on(NoopAgent.execute_skill(SkillExecutionRequest {
            skill_name: "any",
            input: "input".to_string(),
            ctx,
        }));
        assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
    }

    #[test]
    fn test_noop_agent_edge_skills_returns_empty() {
        assert_eq!(
            NoopAgent.skills(AgentSkillsRequest).unwrap().skills.len(),
            0
        );
    }

    #[test]
    fn test_noop_agent_default_role_is_assistant() {
        assert_eq!(
            NoopAgent.supported_role(SupportedRoleRequest).unwrap().role,
            Role::Assistant
        );
    }

    #[test]
    fn test_noop_agent_default_tool_choice_is_auto() {
        assert_eq!(
            NoopAgent
                .tool_choice(ToolChoicePreferenceRequest)
                .unwrap()
                .choice,
            ToolChoice::Auto
        );
    }
}
