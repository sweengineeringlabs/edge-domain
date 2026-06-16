//! Integration tests — `Agent` trait via SAF facade.

use async_trait::async_trait;
use edge_domain_agent::{Agent, AgentError, Skill};
use std::sync::Arc;

struct TestAgent;

#[async_trait]
impl Agent for TestAgent {
    fn id(&self) -> &str {
        "test_agent"
    }

    fn name(&self) -> &str {
        "Test Agent"
    }

    fn description(&self) -> &str {
        "Agent for testing"
    }

    async fn execute_skill(
        &self,
        skill_name: &str,
        _input: String,
    ) -> Result<String, AgentError> {
        match skill_name {
            "success" => Ok("executed".to_string()),
            "fail" => Err(AgentError::ExecutionFailed("deliberate".to_string())),
            _ => Err(AgentError::SkillNotFound(skill_name.to_string())),
        }
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }
}

/// @covers: Agent::id
#[test]
fn trait_agent_happy_id_returns_configured_value() {
    assert_eq!(TestAgent.id(), "test_agent");
}

/// @covers: Agent::name
#[test]
fn trait_agent_happy_name_returns_configured_value() {
    assert_eq!(TestAgent.name(), "Test Agent");
}

/// @covers: Agent::description
#[test]
fn trait_agent_happy_description_returns_configured_value() {
    assert_eq!(TestAgent.description(), "Agent for testing");
}

/// @covers: Agent::execute_skill — success
#[test]
fn trait_agent_happy_execute_skill_success_returns_ok() {
    let result =
        futures::executor::block_on(TestAgent.execute_skill("success", "input".to_string()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "executed");
}

/// @covers: Agent::execute_skill — skill not found error
#[test]
fn trait_agent_error_execute_skill_unknown_returns_skill_not_found() {
    let result =
        futures::executor::block_on(TestAgent.execute_skill("unknown", "input".to_string()));
    assert!(result.is_err());
    match result {
        Err(AgentError::SkillNotFound(name)) => assert_eq!(name, "unknown"),
        _ => panic!("Expected SkillNotFound error"),
    }
}

/// @covers: Agent::execute_skill — execution failure
#[test]
fn trait_agent_error_execute_skill_failure_returns_execution_failed() {
    let result = futures::executor::block_on(TestAgent.execute_skill("fail", "input".to_string()));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::ExecutionFailed(_))));
}

/// @covers: Agent::skills
#[test]
fn trait_agent_edge_skills_returns_empty_list() {
    let skills = TestAgent.skills();
    assert_eq!(skills.len(), 0);
}

/// @covers: AGENT_SVC constant
#[test]
fn svc_agent_svc_happy_constant_equals_agent() {
    assert_eq!(edge_domain_agent::AGENT_SVC, "agent");
}

/// @covers: AGENT_SVC constant validation
#[test]
fn svc_agent_svc_error_constant_not_empty() {
    assert!(!edge_domain_agent::AGENT_SVC.is_empty());
}
