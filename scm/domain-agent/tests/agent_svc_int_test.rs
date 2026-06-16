//! Integration tests for AGENT_SVC constant and Agent trait re-export.

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

/// @covers: AGENT_SVC constant
#[test]
fn svc_agent_svc_happy_constant_equals_agent() {
    assert_eq!(edge_domain_agent::AGENT_SVC, "agent");
}

/// @covers: AGENT_SVC constant
#[test]
fn svc_agent_svc_error_constant_not_empty() {
    assert!(!edge_domain_agent::AGENT_SVC.is_empty());
}

/// @covers: AGENT_SVC constant
#[test]
fn svc_agent_svc_edge_constant_is_valid_identifier() {
    let svc = edge_domain_agent::AGENT_SVC;
    assert!(svc.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
}

/// @covers: Agent trait re-export
#[test]
fn svc_agent_happy_trait_can_be_implemented() {
    let agent: Box<dyn Agent> = Box::new(TestAgent);
    assert_eq!(agent.id(), "test_agent");
}

/// @covers: Agent trait re-export — execute_skill
#[test]
fn svc_agent_happy_execute_skill_success() {
    let result = futures::executor::block_on(TestAgent.execute_skill("success", "input".to_string()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "executed");
}

/// @covers: Agent trait re-export — execute_skill error handling
#[test]
fn svc_agent_error_execute_skill_unknown_skill() {
    let result = futures::executor::block_on(TestAgent.execute_skill("unknown", "input".to_string()));
    assert!(result.is_err());
    match result {
        Err(AgentError::SkillNotFound(name)) => assert_eq!(name, "unknown"),
        _ => panic!("Expected SkillNotFound error"),
    }
}

/// @covers: Agent trait re-export — metadata methods
#[test]
fn svc_agent_happy_metadata_methods_return_strings() {
    let agent = TestAgent;
    assert!(!agent.id().is_empty());
    assert!(!agent.name().is_empty());
    assert!(!agent.description().is_empty());
}
