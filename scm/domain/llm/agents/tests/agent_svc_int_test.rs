#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_SVC constant and Agent trait re-export.

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};
use edge_llm_agent::{
    Agent, AgentDescriptionRequest, AgentError, AgentIdRequest, AgentNameRequest,
    AgentProviderRequest, AgentSkillsRequest, SkillExecutionRequest,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        Box::<ModelInfo>::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

struct TestAgent;

#[async_trait]
impl Agent for TestAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<edge_llm_agent::AgentIdResponse, AgentError> {
        Ok(edge_llm_agent::AgentIdResponse {
            id: "test_agent".to_string(),
        })
    }

    fn name(
        &self,
        _req: AgentNameRequest,
    ) -> Result<edge_llm_agent::AgentNameResponse, AgentError> {
        Ok(edge_llm_agent::AgentNameResponse {
            name: "Test Agent".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<edge_llm_agent::AgentDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::AgentDescriptionResponse {
            description: "Agent for testing".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<edge_llm_agent::SkillExecutionResponse, AgentError> {
        match req.skill_name {
            "success" => Ok(edge_llm_agent::SkillExecutionResponse {
                output: "executed".to_string(),
            }),
            "fail" => Err(AgentError::ExecutionFailed("deliberate".to_string())),
            other => Err(AgentError::SkillNotFound(other.to_string())),
        }
    }

    fn skills(
        &self,
        _req: AgentSkillsRequest,
    ) -> Result<edge_llm_agent::AgentSkillsResponse, AgentError> {
        Ok(edge_llm_agent::AgentSkillsResponse { skills: vec![] })
    }

    fn provider(
        &self,
        _req: AgentProviderRequest,
    ) -> Result<edge_llm_agent::AgentProviderResponse, AgentError> {
        Ok(edge_llm_agent::AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

/// @covers: AGENT_SVC constant
#[test]
fn test_svc_agent_svc_happy_constant_equals_agent() {
    assert_eq!(edge_llm_agent::AGENT_SVC, "agent");
}

/// @covers: AGENT_SVC constant
#[test]
fn test_svc_agent_svc_error_constant_not_empty() {
    assert!(!edge_llm_agent::AGENT_SVC.is_empty());
}

/// @covers: AGENT_SVC constant
#[test]
fn test_svc_agent_svc_edge_constant_is_valid_identifier() {
    let svc = edge_llm_agent::AGENT_SVC;
    assert!(svc.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
}

/// @covers: Agent trait re-export
#[test]
fn test_svc_agent_happy_trait_can_be_implemented() {
    let agent: Box<dyn Agent> = Box::new(TestAgent);
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "test_agent");
}

/// @covers: Agent trait re-export — execute_skill
#[test]
fn test_svc_agent_happy_execute_skill_success() {
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(TestAgent.execute_skill(SkillExecutionRequest {
        skill_name: "success",
        input: "input".to_string(),
        ctx,
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().output, "executed");
}

/// @covers: Agent trait re-export — execute_skill error handling
#[test]
fn test_svc_agent_error_execute_skill_unknown_skill() {
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(TestAgent.execute_skill(SkillExecutionRequest {
        skill_name: "unknown",
        input: "input".to_string(),
        ctx,
    }));
    assert!(result.is_err());
    match result {
        Err(AgentError::SkillNotFound(name)) => assert_eq!(name, "unknown"),
        _ => panic!("Expected SkillNotFound error"),
    }
}

/// @covers: Agent trait re-export — metadata methods
#[test]
fn test_svc_agent_happy_metadata_methods_return_strings() {
    let agent = TestAgent;
    assert!(!agent.id(AgentIdRequest).unwrap().id.is_empty());
    assert!(!agent.name(AgentNameRequest).unwrap().name.is_empty());
    assert!(!agent
        .description(AgentDescriptionRequest)
        .unwrap()
        .description
        .is_empty());
}
