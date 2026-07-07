//! Integration tests for `DefaultAgent` — concrete `Agent` impl backed by provider and skills.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::DirectCommandBus;
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, IdRequest, PatternRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_agent::{
    AgentCreationRequest, AgentError, AgentManager, NoopAgentManager, Skill,
    SkillDescriptionRequest, SkillExecutionRequest, SkillMetadata, SkillMetadataLookupRequest,
    SkillNameRequest, DEFAULT_AGENT_SVC,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderConfig, ProviderNameRequest, StdProvider,
};
use futures::executor::block_on;

fn noop_provider() -> Arc<dyn Provider> {
    Arc::new(StdProvider::new(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    ))
}

/// A minimal skill that echoes `"echo:<input>"`, used as a test double.
struct EchoSkill;

#[async_trait]
impl Handler for EchoSkill {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<edge_domain_handler::IdResponse, HandlerError> {
        Ok(edge_domain_handler::IdResponse {
            id: "echo".to_string(),
        })
    }
    fn pattern(
        &self,
        _req: PatternRequest,
    ) -> Result<edge_domain_handler::PatternResponse, HandlerError> {
        Ok(edge_domain_handler::PatternResponse {
            pattern: "echo".to_string(),
        })
    }

    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        if req.req.is_empty() {
            return Err(HandlerError::ExecutionFailed("empty input".to_string()));
        }
        Ok(format!("echo:{}", req.req))
    }
}

impl Skill for EchoSkill {
    fn name(
        &self,
        _req: SkillNameRequest,
    ) -> Result<edge_llm_agent::SkillNameResponse, AgentError> {
        Ok(edge_llm_agent::SkillNameResponse {
            name: "echo".to_string(),
        })
    }
    fn description(
        &self,
        _req: SkillDescriptionRequest,
    ) -> Result<edge_llm_agent::SkillDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::SkillDescriptionResponse {
            description: "echoes input".to_string(),
        })
    }
    fn metadata(
        &self,
        _req: SkillMetadataLookupRequest,
    ) -> Result<edge_llm_agent::SkillMetadataLookupResponse, AgentError> {
        Ok(edge_llm_agent::SkillMetadataLookupResponse {
            metadata: Box::new(SkillMetadata {
                name: "echo".to_string(),
                description: "echoes input".to_string(),
                input_schema: None,
                output_schema: None,
                async_execution: true,
                long_running: false,
            }),
        })
    }
}

fn echo_skill() -> Arc<dyn Skill<Request = String, Response = String>> {
    Arc::new(EchoSkill)
}

// ── DefaultAgent construction ────────────────────────────────────────────────

/// @covers: default_agent
#[test]
fn test_default_agent_happy_id_name_description_stored() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "agent-1",
            name: "Test Agent",
            description: "for testing",
            provider: noop_provider(),
            skills: vec![],
        })
        .unwrap()
        .agent;
    assert_eq!(
        agent.id(edge_llm_agent::AgentIdRequest).unwrap().id,
        "agent-1"
    );
    assert_eq!(
        agent.name(edge_llm_agent::AgentNameRequest).unwrap().name,
        "Test Agent"
    );
    assert_eq!(
        agent
            .description(edge_llm_agent::AgentDescriptionRequest)
            .unwrap()
            .description,
        "for testing"
    );
}

/// @covers: default_agent
#[test]
fn test_default_agent_happy_provider_accessible() {
    let provider = noop_provider();
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: Arc::clone(&provider),
            skills: vec![],
        })
        .unwrap()
        .agent;
    assert_eq!(
        agent
            .provider(edge_llm_agent::AgentProviderRequest)
            .unwrap()
            .provider
            .name(ProviderNameRequest)
            .unwrap()
            .name,
        provider.name(ProviderNameRequest).unwrap().name
    );
}

/// @covers: default_agent
#[test]
fn test_default_agent_happy_skills_returned() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: noop_provider(),
            skills: vec![echo_skill()],
        })
        .unwrap()
        .agent;
    let skills = agent
        .skills(edge_llm_agent::AgentSkillsRequest)
        .unwrap()
        .skills;
    assert_eq!(skills.len(), 1);
    assert_eq!(skills[0].name(SkillNameRequest).unwrap().name, "echo");
}

// ── execute_skill ────────────────────────────────────────────────────────────

/// @covers: execute_skill
#[test]
fn test_default_agent_happy_execute_skill_routes_to_echo_skill() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: noop_provider(),
            skills: vec![echo_skill()],
        })
        .unwrap()
        .agent;
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = block_on(agent.execute_skill(SkillExecutionRequest {
        skill_name: "echo",
        input: "hello".to_string(),
        ctx,
    }))
    .expect("ok");
    assert_eq!(result.output, "echo:hello");
}

/// @covers: execute_skill
#[test]
fn test_default_agent_error_execute_skill_unknown_returns_skill_not_found() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: noop_provider(),
            skills: vec![echo_skill()],
        })
        .unwrap()
        .agent;
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let err = block_on(agent.execute_skill(SkillExecutionRequest {
        skill_name: "missing",
        input: "x".to_string(),
        ctx,
    }))
    .expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

/// @covers: execute_skill
#[test]
fn test_default_agent_error_execute_skill_bad_input_propagates_execution_failed() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: noop_provider(),
            skills: vec![echo_skill()],
        })
        .unwrap()
        .agent;
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let err = block_on(agent.execute_skill(SkillExecutionRequest {
        skill_name: "echo",
        input: String::new(),
        ctx,
    }))
    .expect_err("should fail on empty input");
    assert!(matches!(err, AgentError::ExecutionFailed(_)));
}

/// @covers: execute_skill
#[test]
fn test_default_agent_edge_execute_skill_no_skills_returns_not_found() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: noop_provider(),
            skills: vec![],
        })
        .unwrap()
        .agent;
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let err = block_on(agent.execute_skill(SkillExecutionRequest {
        skill_name: "anything",
        input: "x".to_string(),
        ctx,
    }))
    .expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

// ── DEFAULT_AGENT_SVC constant ───────────────────────────────────────────────

/// @covers: DEFAULT_AGENT_SVC
#[test]
fn test_default_agent_svc_constant_is_default_agent() {
    assert_eq!(DEFAULT_AGENT_SVC, "default_agent");
}
