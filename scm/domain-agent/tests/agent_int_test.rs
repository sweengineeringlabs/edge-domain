//! Integration tests — `Agent` trait.

use async_trait::async_trait;
use edge_domain_agent::{Agent, AgentError, Skill};
use std::sync::Arc;

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

    async fn execute_skill(
        &self,
        skill_name: &str,
        input: String,
    ) -> Result<String, AgentError> {
        Ok(format!("{}:{}", skill_name, input))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
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

    async fn execute_skill(
        &self,
        _skill_name: &str,
        _input: String,
    ) -> Result<String, AgentError> {
        Err(AgentError::ExecutionFailed("deliberate failure".to_string()))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }
}

/// @covers: Agent::id
#[test]
fn trait_agent_happy_id_returns_configured_id() {
    assert_eq!(SuccessAgent.id(), "success");
}

/// @covers: Agent::id — multiple implementations
#[test]
fn trait_agent_happy_id_differs_by_implementation() {
    assert_ne!(SuccessAgent.id(), FailingAgent.id());
}

/// @covers: Agent::name
#[test]
fn trait_agent_happy_name_returns_configured_name() {
    assert_eq!(SuccessAgent.name(), "Success Agent");
}

/// @covers: Agent::description
#[test]
fn trait_agent_happy_description_returns_configured_description() {
    assert_eq!(SuccessAgent.description(), "Always succeeds");
}

/// @covers: Agent::execute_skill — success case
#[test]
fn trait_agent_happy_execute_skill_success_returns_ok_response() {
    let result = futures::executor::block_on(
        SuccessAgent.execute_skill("analyze", "test_input".to_string()),
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "analyze:test_input");
}

/// @covers: Agent::execute_skill — failure case
#[test]
fn trait_agent_error_execute_skill_failure_returns_execution_failed() {
    let result = futures::executor::block_on(
        FailingAgent.execute_skill("any_skill", "input".to_string()),
    );
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
fn trait_agent_happy_execute_skill_preserves_input() {
    let result = futures::executor::block_on(
        SuccessAgent.execute_skill("skill", "preserved".to_string()),
    );
    assert_eq!(result.unwrap(), "skill:preserved");
}

/// @covers: Agent::skills — empty implementation
#[test]
fn trait_agent_edge_skills_returns_empty_list() {
    assert_eq!(SuccessAgent.skills().len(), 0);
}

/// @covers: Agent::skill — delegates to skills()
#[test]
fn trait_agent_error_skill_returns_skill_not_found_when_empty() {
    let result = SuccessAgent.skill("nonexistent");
    assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
}

/// @covers: Agent — all methods work together
#[test]
fn trait_agent_happy_all_methods_together_consistent() {
    assert!(!SuccessAgent.id().is_empty());
    assert!(!SuccessAgent.name().is_empty());
    assert!(!SuccessAgent.description().is_empty());
    assert_eq!(SuccessAgent.id(), "success");
}
