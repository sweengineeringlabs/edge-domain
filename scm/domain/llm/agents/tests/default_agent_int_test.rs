//! Integration tests for `DefaultAgent` — concrete `Agent` impl backed by provider and skills.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_observe::StdObserveFactory;
use edge_llm_agent::{
    AgentError, AgentManager, NoopAgentManager, Skill, SkillMetadata, DEFAULT_AGENT_SVC,
};
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

/// A minimal skill that echoes `"echo:<input>"`, used as a test double.
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
            return Err(HandlerError::ExecutionFailed("empty input".to_string()));
        }
        Ok(format!("echo:{input}"))
    }
}

impl Skill for EchoSkill {
    fn name(&self) -> &str {
        "echo"
    }
    fn description(&self) -> &str {
        "echoes input"
    }
    fn metadata(&self) -> SkillMetadata {
        SkillMetadata {
            name: "echo".to_string(),
            description: "echoes input".to_string(),
            input_schema: None,
            output_schema: None,
            async_execution: true,
            long_running: false,
        }
    }
}

fn echo_skill() -> Arc<dyn Skill<Request = String, Response = String>> {
    Arc::new(EchoSkill)
}

// ── DefaultAgent construction ────────────────────────────────────────────────

/// @covers: default_agent
#[test]
fn test_default_agent_happy_id_name_description_stored() {
    let agent = NoopAgentManager.default_agent(
        "agent-1",
        "Test Agent",
        "for testing",
        noop_provider(),
        vec![],
    );
    assert_eq!(agent.id(), "agent-1");
    assert_eq!(agent.name(), "Test Agent");
    assert_eq!(agent.description(), "for testing");
}

/// @covers: default_agent
#[test]
fn test_default_agent_happy_provider_accessible() {
    let provider = noop_provider();
    let agent = NoopAgentManager.default_agent("a", "A", "desc", Arc::clone(&provider), vec![]);
    assert_eq!(agent.provider().name(), provider.name());
}

/// @covers: default_agent
#[test]
fn test_default_agent_happy_skills_returned() {
    let agent =
        NoopAgentManager.default_agent("a", "A", "desc", noop_provider(), vec![echo_skill()]);
    assert_eq!(agent.skills().len(), 1);
    assert_eq!(agent.skills()[0].name(), "echo");
}

// ── execute_skill ────────────────────────────────────────────────────────────

/// @covers: execute_skill
#[test]
fn test_default_agent_happy_execute_skill_routes_to_echo_skill() {
    let agent =
        NoopAgentManager.default_agent("a", "A", "desc", noop_provider(), vec![echo_skill()]);
    let result = block_on(agent.execute_skill("echo", "hello".to_string())).expect("ok");
    assert_eq!(result, "echo:hello");
}

/// @covers: execute_skill
#[test]
fn test_default_agent_error_execute_skill_unknown_returns_skill_not_found() {
    let agent =
        NoopAgentManager.default_agent("a", "A", "desc", noop_provider(), vec![echo_skill()]);
    let err = block_on(agent.execute_skill("missing", "x".to_string())).expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

/// @covers: execute_skill
#[test]
fn test_default_agent_error_execute_skill_bad_input_propagates_execution_failed() {
    let agent =
        NoopAgentManager.default_agent("a", "A", "desc", noop_provider(), vec![echo_skill()]);
    let err = block_on(agent.execute_skill("echo", String::new()))
        .expect_err("should fail on empty input");
    assert!(matches!(err, AgentError::ExecutionFailed(_)));
}

/// @covers: execute_skill
#[test]
fn test_default_agent_edge_execute_skill_no_skills_returns_not_found() {
    let agent = NoopAgentManager.default_agent("a", "A", "desc", noop_provider(), vec![]);
    let err = block_on(agent.execute_skill("anything", "x".to_string())).expect_err("should fail");
    assert!(matches!(err, AgentError::SkillNotFound(_)));
}

// ── DEFAULT_AGENT_SVC constant ───────────────────────────────────────────────

/// @covers: DEFAULT_AGENT_SVC
#[test]
fn test_default_agent_svc_constant_is_default_agent() {
    assert_eq!(DEFAULT_AGENT_SVC, "default_agent");
}
