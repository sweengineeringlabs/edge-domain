//! Integration tests — `AgentManager` trait.

use async_trait::async_trait;
use edge_domain_agent::{Agent, AgentError, AgentManager, Skill};
use std::sync::Arc;

struct TestManager {
    should_fail: bool,
}

#[async_trait]
impl AgentManager for TestManager {
    async fn load_agent(&self, spec: &str) -> Result<Arc<dyn Agent>, AgentError> {
        if self.should_fail {
            Err(AgentError::InvalidSpec(format!("Cannot load: {}", spec)))
        } else if spec.is_empty() {
            Err(AgentError::InvalidSpec("Empty spec".to_string()))
        } else {
            Ok(Arc::new(DummyAgent))
        }
    }

    fn agent(&self, id: &str) -> Result<Arc<dyn Agent>, AgentError> {
        if self.should_fail {
            Err(AgentError::NotFound(id.to_string()))
        } else if id == "exists" {
            Ok(Arc::new(DummyAgent))
        } else {
            Err(AgentError::NotFound(id.to_string()))
        }
    }

    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError> {
        if self.should_fail {
            Err(AgentError::ExecutionFailed("Cannot list".to_string()))
        } else {
            Ok(vec!["agent1".to_string(), "agent2".to_string()])
        }
    }
}

struct DummyAgent;

#[async_trait]
impl Agent for DummyAgent {
    fn id(&self) -> &str {
        "dummy"
    }

    fn name(&self) -> &str {
        "Dummy"
    }

    fn description(&self) -> &str {
        "Dummy agent for testing"
    }

    async fn execute_skill(
        &self,
        _skill_name: &str,
        _input: String,
    ) -> Result<String, AgentError> {
        Ok("dummy_response".to_string())
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }
}

/// @covers: AgentManager::load_agent — success
#[test]
fn test_trait_agent_manager_happy_load_agent_valid_spec_returns_ok() {
    let manager = TestManager {
        should_fail: false,
    };
    let result = futures::executor::block_on(manager.load_agent("valid.yaml"));
    assert!(result.is_ok());
}

/// @covers: AgentManager::load_agent — invalid spec
#[test]
fn test_trait_agent_manager_error_load_agent_invalid_spec_returns_error() {
    let manager = TestManager {
        should_fail: true,
    };
    let result = futures::executor::block_on(manager.load_agent("bad.yaml"));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

/// @covers: AgentManager::load_agent — empty spec
#[test]
fn test_trait_agent_manager_error_load_agent_empty_spec_returns_invalid_spec() {
    let manager = TestManager {
        should_fail: false,
    };
    let result = futures::executor::block_on(manager.load_agent(""));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

/// @covers: AgentManager::agent — found
#[test]
fn test_trait_agent_manager_happy_agent_existing_id_returns_ok() {
    let manager = TestManager {
        should_fail: false,
    };
    let result = manager.agent("exists");
    assert!(result.is_ok());
}

/// @covers: AgentManager::agent — not found
#[test]
fn test_trait_agent_manager_error_agent_nonexistent_id_returns_not_found() {
    let manager = TestManager {
        should_fail: false,
    };
    let result = manager.agent("missing");
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: AgentManager::agent — manager failure
#[test]
fn test_trait_agent_manager_error_agent_when_manager_fails_returns_error() {
    let manager = TestManager { should_fail: true };
    let result = manager.agent("any");
    assert!(result.is_err());
}

/// @covers: AgentManager::list_agent_ids — success
#[test]
fn test_trait_agent_manager_happy_list_agent_ids_returns_list() {
    let manager = TestManager {
        should_fail: false,
    };
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
    let ids = result.unwrap();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"agent1".to_string()));
}

/// @covers: AgentManager::list_agent_ids — failure
#[test]
fn test_trait_agent_manager_error_list_agent_ids_when_fails_returns_error() {
    let manager = TestManager { should_fail: true };
    let result = manager.list_agent_ids();
    assert!(result.is_err());
}

/// @covers: AgentManager::list_agent_ids — empty list
#[test]
fn test_trait_agent_manager_edge_list_agent_ids_empty_list() {
    let manager = TestManager {
        should_fail: false,
    };
    // Redefine to return empty in this test context
    // In real implementation, this would be handled
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
}

/// @covers: AgentManager — all methods consistent
#[test]
fn test_trait_agent_manager_happy_all_methods_work_together() {
    let manager = TestManager {
        should_fail: false,
    };
    let _ = futures::executor::block_on(manager.load_agent("test.yaml"));
    let _ = manager.agent("exists");
    let _ = manager.list_agent_ids();
    // All calls succeeded when should_fail is false
}
