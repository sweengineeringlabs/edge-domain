//! Integration tests for `AgentManager::agent_handler` and `AgentManager::default_agent`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, IdRequest, PatternRequest,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::{SecurityBootstrap, SecurityServices};
use edge_llm_agent::{
    AgentCreationRequest, AgentError, AgentHandlerRequest, AgentManager, NoopAgentManager, Skill,
    SkillDescriptionRequest, SkillExecutionRequest, SkillMetadata, SkillMetadataLookupRequest,
    SkillNameRequest,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
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
            return Err(HandlerError::ExecutionFailed("empty".to_string()));
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
            description: "echoes".to_string(),
        })
    }
    fn metadata(
        &self,
        _req: SkillMetadataLookupRequest,
    ) -> Result<edge_llm_agent::SkillMetadataLookupResponse, AgentError> {
        Ok(edge_llm_agent::SkillMetadataLookupResponse {
            metadata: Box::new(SkillMetadata {
                name: "echo".into(),
                description: "echoes".into(),
                input_schema: None,
                output_schema: None,
                async_execution: true,
                long_running: false,
            }),
        })
    }
}

// ── agent_handler ─────────────────────────────────────────────────────────────

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_routes_input_to_named_skill_happy() {
    let h = NoopAgentManager
        .agent_handler(AgentHandlerRequest {
            skill: "code_review",
        })
        .unwrap()
        .handler;
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &*h,
        ExecutionRequest {
            req: "diff".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("handler ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_empty_input_returns_error() {
    let h = NoopAgentManager
        .agent_handler(AgentHandlerRequest {
            skill: "code_review",
        })
        .unwrap()
        .handler;
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    assert!(block_on(Handler::execute(
        &*h,
        ExecutionRequest {
            req: String::new(),
            ctx: &ctx,
        }
    ))
    .is_err());
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_id_and_pattern_are_stable_edge() {
    let h = NoopAgentManager
        .agent_handler(AgentHandlerRequest { skill: "any_skill" })
        .unwrap()
        .handler;
    assert_eq!(
        Handler::id(&*h, IdRequest).unwrap().id,
        "agent.execute_skill"
    );
    assert_eq!(
        Handler::pattern(&*h, PatternRequest).unwrap().pattern,
        "agent/execute_skill"
    );
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_targets_different_skill_names_happy() {
    let h = NoopAgentManager
        .agent_handler(AgentHandlerRequest { skill: "planning" })
        .unwrap()
        .handler;
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &*h,
        ExecutionRequest {
            req: "a task".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert_eq!(out, "planning:a task");
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_empty_skill_name_preserved_edge() {
    let h = NoopAgentManager
        .agent_handler(AgentHandlerRequest { skill: "" })
        .unwrap()
        .handler;
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let out = block_on(Handler::execute(
        &*h,
        ExecutionRequest {
            req: "input".to_string(),
            ctx: &ctx,
        },
    ))
    .expect("ok");
    assert_eq!(out, ":input");
}

// ── default_agent ─────────────────────────────────────────────────────────────

/// @covers: AgentManager::default_agent
#[test]
fn test_default_agent_executes_registered_skill_happy() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "a",
            name: "A",
            description: "desc",
            provider: noop_provider(),
            skills: vec![Arc::new(EchoSkill)],
        })
        .unwrap()
        .agent;
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = block_on(agent.execute_skill(SkillExecutionRequest {
        skill_name: "echo",
        input: "hi".to_string(),
        ctx,
    }))
    .expect("ok");
    assert_eq!(result.output, "echo:hi");
}

/// @covers: AgentManager::default_agent
#[test]
fn test_default_agent_missing_skill_returns_not_found_error() {
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
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let err = block_on(agent.execute_skill(SkillExecutionRequest {
        skill_name: "ghost",
        input: "x".to_string(),
        ctx,
    }))
    .expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

/// @covers: AgentManager::default_agent
#[test]
fn test_default_agent_identity_fields_round_trip_edge() {
    let agent = NoopAgentManager
        .default_agent(AgentCreationRequest {
            id: "edge-id",
            name: "edge-name",
            description: "edge-desc",
            provider: noop_provider(),
            skills: vec![],
        })
        .unwrap()
        .agent;
    assert_eq!(
        agent.id(edge_llm_agent::AgentIdRequest).unwrap().id,
        "edge-id"
    );
    assert_eq!(
        agent.name(edge_llm_agent::AgentNameRequest).unwrap().name,
        "edge-name"
    );
    assert_eq!(
        agent
            .description(edge_llm_agent::AgentDescriptionRequest)
            .unwrap()
            .description,
        "edge-desc"
    );
}
