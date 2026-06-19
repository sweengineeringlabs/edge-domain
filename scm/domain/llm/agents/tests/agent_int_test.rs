#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests — `Agent` trait.

use async_trait::async_trait;
use edge_llm_agent::{Agent, AgentError, MessageContent, Role, Skill, ToolChoice};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderConfig, ProviderFactory, StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    Arc::new(StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
    ))
}

struct SuccessAgent;

#[async_trait]
impl Agent for SuccessAgent {
    fn id(&self) -> &str {
        "success"
    }

    fn name(&self) -> &str {
        "Success Agent"
    }

    fn description(&self) -> &str {
        "Always succeeds"
    }

    async fn execute_skill(&self, skill_name: &str, input: String) -> Result<String, AgentError> {
        Ok(format!("{}:{}", skill_name, input))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

struct FailingAgent;

#[async_trait]
impl Agent for FailingAgent {
    fn id(&self) -> &str {
        "failing"
    }

    fn name(&self) -> &str {
        "Failing Agent"
    }

    fn description(&self) -> &str {
        "Always fails"
    }

    async fn execute_skill(&self, _skill_name: &str, _input: String) -> Result<String, AgentError> {
        Err(AgentError::ExecutionFailed(
            "deliberate failure".to_string(),
        ))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

/// @covers: Agent::id
#[test]
fn test_trait_agent_happy_id_returns_configured_id() {
    assert_eq!(SuccessAgent.id(), "success");
}

/// @covers: Agent::id — multiple implementations
#[test]
fn test_trait_agent_happy_id_differs_by_implementation() {
    assert_ne!(SuccessAgent.id(), FailingAgent.id());
}

/// @covers: Agent::name
#[test]
fn test_trait_agent_happy_name_returns_configured_name() {
    assert_eq!(SuccessAgent.name(), "Success Agent");
}

/// @covers: Agent::description
#[test]
fn test_trait_agent_happy_description_returns_configured_description() {
    assert_eq!(SuccessAgent.description(), "Always succeeds");
}

/// @covers: Agent::execute_skill — success case
#[test]
fn test_trait_agent_happy_execute_skill_success_returns_ok_response() {
    let result = futures::executor::block_on(
        SuccessAgent.execute_skill("analyze", "test_input".to_string()),
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "analyze:test_input");
}

/// @covers: Agent::execute_skill — failure case
#[test]
fn test_trait_agent_error_execute_skill_failure_returns_execution_failed() {
    let result =
        futures::executor::block_on(FailingAgent.execute_skill("any_skill", "input".to_string()));
    assert!(result.is_err());
    match result {
        Err(AgentError::ExecutionFailed(msg)) => {
            assert_eq!(msg, "deliberate failure");
        }
        _ => panic!("Expected ExecutionFailed error"),
    }
}

/// @covers: Agent::execute_skill — input is passed through
#[test]
fn test_trait_agent_happy_execute_skill_preserves_input() {
    let result =
        futures::executor::block_on(SuccessAgent.execute_skill("skill", "preserved".to_string()));
    assert_eq!(result.unwrap(), "skill:preserved");
}

/// @covers: Agent::skills — empty implementation
#[test]
fn test_trait_agent_edge_skills_returns_empty_list() {
    assert_eq!(SuccessAgent.skills().len(), 0);
}

/// @covers: Agent::skill — delegates to skills()
#[test]
fn test_trait_agent_error_skill_returns_skill_not_found_when_empty() {
    let result = SuccessAgent.skill("nonexistent");
    assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
}

/// @covers: Agent — all methods work together
#[test]
fn test_trait_agent_happy_all_methods_together_consistent() {
    assert!(!SuccessAgent.id().is_empty());
    assert!(!SuccessAgent.name().is_empty());
    assert!(!SuccessAgent.description().is_empty());
    assert_eq!(SuccessAgent.id(), "success");
}

// --- send ---

/// @covers: send
#[test]
fn test_send_accepts_message_happy() {
    let agent = SuccessAgent;
    let msg = agent.message_builder().content("hello").build();
    assert_eq!(agent.send(msg), 1);
}

/// @covers: send
#[test]
fn test_send_default_is_stateless_error() {
    // The default impl does not accumulate: each send reports a single message.
    let agent = SuccessAgent;
    let first = agent.send(agent.message_builder().content("a").build());
    let second = agent.send(agent.message_builder().content("b").build());
    assert_eq!(first, second);
}

/// @covers: send
#[test]
fn test_send_empty_content_edge() {
    let agent = SuccessAgent;
    let msg = agent.message_builder().build();
    assert_eq!(agent.send(msg), 1);
}

// --- supported_role ---

/// @covers: supported_role
#[test]
fn test_supported_role_defaults_assistant_happy() {
    assert_eq!(SuccessAgent.supported_role(), Role::Assistant);
}

/// @covers: supported_role
#[test]
fn test_supported_role_is_not_user_error() {
    assert_ne!(SuccessAgent.supported_role(), Role::User);
}

/// @covers: supported_role
#[test]
fn test_supported_role_consistent_across_impls_edge() {
    assert_eq!(SuccessAgent.supported_role(), FailingAgent.supported_role());
}

// --- tool_choice ---

/// @covers: tool_choice
#[test]
fn test_tool_choice_defaults_auto_happy() {
    assert_eq!(SuccessAgent.tool_choice(), ToolChoice::Auto);
}

/// @covers: tool_choice
#[test]
fn test_tool_choice_is_not_none_error() {
    assert_ne!(SuccessAgent.tool_choice(), ToolChoice::None);
}

/// @covers: tool_choice
#[test]
fn test_tool_choice_consistent_across_impls_edge() {
    assert_eq!(SuccessAgent.tool_choice(), FailingAgent.tool_choice());
}

// --- message_builder ---

/// @covers: message_builder
#[test]
fn test_message_builder_builds_message_happy() {
    let msg = SuccessAgent.message_builder().content("hi").build();
    assert_eq!(msg.role, Role::User);
}

/// @covers: message_builder
#[test]
fn test_message_builder_role_override_error() {
    let msg = SuccessAgent
        .message_builder()
        .role(Role::System)
        .content("sys")
        .build();
    assert_ne!(msg.role, Role::User);
}

/// @covers: message_builder
#[test]
fn test_message_builder_default_content_empty_edge() {
    let msg = SuccessAgent.message_builder().build();
    assert_eq!(msg.content, MessageContent::text(""));
}

// --- provider ---

/// @covers: Agent::provider
#[test]
fn test_provider_returns_arc_dyn_provider_happy() {
    let _p: Arc<dyn Provider> = SuccessAgent.provider();
}

/// @covers: Agent::provider
#[test]
fn test_provider_health_check_ok_happy() {
    assert!(SuccessAgent.provider().health_check().is_ok());
}

/// @covers: Agent::provider
#[test]
fn test_provider_distinct_per_impl_error() {
    // Two different agent types may back different providers.
    let p1 = SuccessAgent.provider();
    let p2 = FailingAgent.provider();
    // Both healthy — confirming each returns a usable provider.
    assert!(p1.health_check().is_ok());
    assert!(p2.health_check().is_ok());
}

/// @covers: Agent::provider
#[test]
fn test_provider_completer_accessible_from_provider_edge() {
    // Completer is reachable through the provider seam without naming ProviderCore.
    let p = SuccessAgent.provider();
    let _completer = p.completer();
}
