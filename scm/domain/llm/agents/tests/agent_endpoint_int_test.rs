//! Integration tests for `AgentManager::agent_handler` and `AgentManager::default_agent`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_observer::StdObserveFactory;
use edge_domain_security::SecurityContext;
use edge_llm_agent::{AgentError, AgentManager, NoopAgentManager, Skill, SkillMetadata};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
};
use futures::executor::block_on;

fn noop_provider() -> Arc<dyn Provider> {
    StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

struct EchoSkill;
#[async_trait]
impl Handler for EchoSkill {
    type Request = String;
    type Response = String;
    fn id(&self) -> &str {
        "echo"
    }
    fn pattern(&self) -> &str {
        "echo"
    }
    async fn execute(
        &self,
        input: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        if input.is_empty() {
            return Err(HandlerError::ExecutionFailed("empty".to_string()));
        }
        Ok(format!("echo:{input}"))
    }
}
impl Skill for EchoSkill {
    fn name(&self) -> &str {
        "echo"
    }
    fn description(&self) -> &str {
        "echoes"
    }
    fn metadata(&self) -> SkillMetadata {
        SkillMetadata {
            name: "echo".into(),
            description: "echoes".into(),
            input_schema: None,
            output_schema: None,
            async_execution: true,
            long_running: false,
        }
    }
}

// ── agent_handler ─────────────────────────────────────────────────────────────

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_routes_input_to_named_skill_happy() {
    let h = NoopAgentManager.agent_handler("code_review");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let out = block_on(Handler::execute(&*h, "diff".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_empty_input_returns_error() {
    let h = NoopAgentManager.agent_handler("code_review");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    assert!(block_on(Handler::execute(&*h, String::new(), ctx)).is_err());
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_id_and_pattern_are_stable_edge() {
    let h = NoopAgentManager.agent_handler("any_skill");
    assert_eq!(Handler::id(&*h), "agent.execute_skill");
    assert_eq!(Handler::pattern(&*h), "agent/execute_skill");
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_targets_different_skill_names_happy() {
    let h = NoopAgentManager.agent_handler("planning");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let out = block_on(Handler::execute(&*h, "a task".to_string(), ctx)).expect("ok");
    assert_eq!(out, "planning:a task");
}

/// @covers: AgentManager::agent_handler
#[test]
fn test_agent_handler_empty_skill_name_preserved_edge() {
    let h = NoopAgentManager.agent_handler("");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let out = block_on(Handler::execute(&*h, "input".to_string(), ctx)).expect("ok");
    assert_eq!(out, ":input");
}

// ── default_agent ─────────────────────────────────────────────────────────────

/// @covers: AgentManager::default_agent
#[test]
fn test_default_agent_executes_registered_skill_happy() {
    let agent = NoopAgentManager.default_agent(
        "a",
        "A",
        "desc",
        noop_provider(),
        vec![Arc::new(EchoSkill)],
    );
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let result = block_on(agent.execute_skill("echo", "hi".to_string(), ctx)).expect("ok");
    assert_eq!(result, "echo:hi");
}

/// @covers: AgentManager::default_agent
#[test]
fn test_default_agent_missing_skill_returns_not_found_error() {
    let agent = NoopAgentManager.default_agent("a", "A", "desc", noop_provider(), vec![]);
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observe_context();
    let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
    let err =
        block_on(agent.execute_skill("ghost", "x".to_string(), ctx)).expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

/// @covers: AgentManager::default_agent
#[test]
fn test_default_agent_identity_fields_round_trip_edge() {
    let agent = NoopAgentManager.default_agent(
        "edge-id",
        "edge-name",
        "edge-desc",
        noop_provider(),
        vec![],
    );
    assert_eq!(agent.id(), "edge-id");
    assert_eq!(agent.name(), "edge-name");
    assert_eq!(agent.description(), "edge-desc");
}
