//! Integration tests for `AgentEndpoint` — `agent_handler` and `default_agent` factories.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_security::SecurityContext;
use edge_llm_agent::{AgentEndpoint, AgentError, NoopAgentManager, Skill, SkillMetadata};
use edge_llm_provider::{EchoProviderCompleter, ModelInfo, Provider, ProviderConfig, ProviderFactory, StdProviderFactory};
use futures::executor::block_on;

fn noop_provider() -> Arc<dyn Provider> {
    Arc::new(StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
    ))
}

struct EchoSkill;
#[async_trait]
impl Handler for EchoSkill {
    type Request = String;
    type Response = String;
    fn id(&self) -> &str { "echo" }
    fn pattern(&self) -> &str { "echo" }
    async fn execute(&self, input: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        if input.is_empty() { return Err(HandlerError::ExecutionFailed("empty".to_string())); }
        Ok(format!("echo:{input}"))
    }
}
impl Skill for EchoSkill {
    fn name(&self) -> &str { "echo" }
    fn description(&self) -> &str { "echoes" }
    fn metadata(&self) -> SkillMetadata {
        SkillMetadata { name: "echo".into(), description: "echoes".into(), input_schema: None, output_schema: None, async_execution: true, long_running: false }
    }
}

/// @covers: agent_handler (Handler face) — runs core under a request context
#[test]
fn test_handler_execute_returns_skill_colon_input_happy() {
    let h = NoopAgentManager::agent_handler("code_review");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "diff".to_string(), ctx)).expect("handler ok");
    assert_eq!(out, "code_review:diff");
}

/// @covers: agent_handler — dispatch id is stable
#[test]
fn test_handler_id_is_stable_edge() {
    let h = NoopAgentManager::agent_handler("any_skill");
    assert_eq!(Handler::id(&h), "agent.execute_skill");
}

/// @covers: agent_handler — pattern is stable
#[test]
fn test_handler_pattern_is_stable_edge() {
    let h = NoopAgentManager::agent_handler("any_skill");
    assert_eq!(Handler::pattern(&h), "agent/execute_skill");
}

/// @covers: agent_handler — empty input surfaces a handler error
#[test]
fn test_handler_execute_empty_input_returns_error() {
    let h = NoopAgentManager::agent_handler("code_review");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    assert!(block_on(Handler::execute(&h, String::new(), ctx)).is_err());
}

/// @covers: agent_handler — targets the named skill in its output
#[test]
fn test_handler_targets_named_skill_happy() {
    let h = NoopAgentManager::agent_handler("planning");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "a task".to_string(), ctx)).expect("ok");
    assert_eq!(out, "planning:a task");
}

/// @covers: agent_handler — edge: empty skill name is preserved verbatim
#[test]
fn test_handler_empty_skill_name_edge() {
    let h = NoopAgentManager::agent_handler("");
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let ctx = HandlerContext::new(&security, &commands);
    let out = block_on(Handler::execute(&h, "input".to_string(), ctx)).expect("ok");
    assert_eq!(out, ":input");
}

// ── default_agent ────────────────────────────────────────────────────────────

/// @covers: default_agent
#[test]
fn test_default_agent_happy_executes_registered_skill() {
    let agent = NoopAgentManager::default_agent(
        "a", "A", "desc", noop_provider(),
        vec![Arc::new(EchoSkill)],
    );
    let result = block_on(agent.execute_skill("echo", "hi".to_string())).expect("ok");
    assert_eq!(result, "echo:hi");
}

/// @covers: default_agent
#[test]
fn test_default_agent_error_missing_skill_returns_not_found() {
    let agent = NoopAgentManager::default_agent(
        "a", "A", "desc", noop_provider(), vec![],
    );
    let err = block_on(agent.execute_skill("ghost", "x".to_string())).expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

/// @covers: default_agent
#[test]
fn test_default_agent_edge_identity_fields_round_trip() {
    let agent = NoopAgentManager::default_agent(
        "edge-id", "edge-name", "edge-desc", noop_provider(), vec![],
    );
    assert_eq!(agent.id(), "edge-id");
    assert_eq!(agent.name(), "edge-name");
    assert_eq!(agent.description(), "edge-desc");
}
