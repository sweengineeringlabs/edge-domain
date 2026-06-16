//! Integration tests for AGENT_REGISTRY_SVC constant and AgentRegistry trait re-export.

use async_trait::async_trait;
use edge_llm_agent::{Agent, AgentError, AgentMetadata, AgentRegistry, Skill};
use edge_domain_registry::Registry;
use std::sync::{Arc, Mutex};

struct TestAgent {
    id: String,
}

#[async_trait]
impl Agent for TestAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        "Test Agent"
    }

    fn description(&self) -> &str {
        "Test agent for registry testing"
    }

    async fn execute_skill(
        &self,
        _skill_name: &str,
        _input: String,
    ) -> Result<String, AgentError> {
        Ok("result".to_string())
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }
}

struct TestAgentRegistry {
    agents: Mutex<std::collections::HashMap<String, (Arc<dyn Agent>, AgentMetadata)>>,
}

impl TestAgentRegistry {
    fn new() -> Self {
        TestAgentRegistry {
            agents: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl Registry for TestAgentRegistry {
    type Value = dyn Agent;

    fn register(&self, id: &str, value: Arc<Self::Value>) {
        self.agents
            .lock()
            .unwrap()
            .insert(id.to_string(), (value, create_dummy_metadata(id)));
    }

    fn try_register(&self, id: &str, value: Arc<Self::Value>) -> Result<(), edge_domain_registry::RegistryError> {
        let mut agents = self.agents.lock().unwrap();
        if agents.contains_key(id) {
            Err(edge_domain_registry::RegistryError::DuplicateId(id.to_string()))
        } else {
            agents.insert(id.to_string(), (value, create_dummy_metadata(id)));
            Ok(())
        }
    }

    fn deregister(&self, id: &str) -> bool {
        self.agents.lock().unwrap().remove(id).is_some()
    }

    fn get(&self, id: &str) -> Option<Arc<Self::Value>> {
        self.agents
            .lock()
            .unwrap()
            .get(id)
            .map(|(agent, _)| agent.clone())
    }

    fn list_ids(&self) -> Vec<String> {
        self.agents.lock().unwrap().keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.agents.lock().unwrap().len()
    }
}

impl AgentRegistry for TestAgentRegistry {
    fn metadata(&self, id: &str) -> Result<AgentMetadata, AgentError> {
        self.agents
            .lock()
            .unwrap()
            .get(id)
            .map(|(_, meta)| meta.clone())
            .ok_or_else(|| AgentError::NotFound(id.to_string()))
    }
}

fn create_dummy_metadata(id: &str) -> AgentMetadata {
    AgentMetadata {
        id: id.to_string(),
        name: format!("Agent {}", id),
        description: "Test agent".to_string(),
        version: "1.0.0".to_string(),
        skills: vec![],
        patterns: vec![],
    }
}

/// @covers: AGENT_REGISTRY_SVC constant
#[test]
fn test_svc_agent_registry_svc_happy_constant_equals_agent_registry() {
    assert_eq!(edge_llm_agent::AGENT_REGISTRY_SVC, "agent_registry");
}

/// @covers: AGENT_REGISTRY_SVC constant
#[test]
fn test_svc_agent_registry_svc_error_constant_not_empty() {
    assert!(!edge_llm_agent::AGENT_REGISTRY_SVC.is_empty());
}

/// @covers: AGENT_REGISTRY_SVC constant
#[test]
fn test_svc_agent_registry_svc_edge_constant_is_valid_identifier() {
    let svc = edge_llm_agent::AGENT_REGISTRY_SVC;
    assert!(svc.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
}

/// @covers: AgentRegistry trait re-export
#[test]
fn test_svc_agent_registry_happy_trait_can_be_implemented() {
    let registry = TestAgentRegistry::new();
    let agent = Arc::new(TestAgent {
        id: "test".to_string(),
    });
    registry.register("test", agent);
    let retrieved = registry.get("test");
    assert!(retrieved.is_some());
}

/// @covers: AgentRegistry trait re-export — inherits Registry::register
#[test]
fn test_svc_agent_registry_happy_register_stores_agent() {
    let registry = TestAgentRegistry::new();
    let agent = Arc::new(TestAgent {
        id: "agent1".to_string(),
    });
    registry.register("agent1", agent);
    let retrieved = registry.get("agent1");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id(), "agent1");
}

/// @covers: AgentRegistry trait re-export — inherits Registry::get
#[test]
fn test_svc_agent_registry_error_get_nonexistent_agent() {
    let registry = TestAgentRegistry::new();
    let result = registry.get("nonexistent");
    assert!(result.is_none());
}

/// @covers: AgentRegistry trait re-export — inherits Registry::list_ids
#[test]
fn test_svc_agent_registry_happy_list_ids_returns_all_agents() {
    let registry = TestAgentRegistry::new();
    let agent1 = Arc::new(TestAgent {
        id: "agent1".to_string(),
    });
    let agent2 = Arc::new(TestAgent {
        id: "agent2".to_string(),
    });
    registry.register("agent1", agent1);
    registry.register("agent2", agent2);
    let ids = registry.list_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"agent1".to_string()));
    assert!(ids.contains(&"agent2".to_string()));
}

/// @covers: AgentRegistry trait re-export — metadata method
#[test]
fn test_svc_agent_registry_happy_metadata_returns_agent_info() {
    let registry = TestAgentRegistry::new();
    let agent = Arc::new(TestAgent {
        id: "test_agent".to_string(),
    });
    registry.register("test_agent", agent);
    let metadata = registry.metadata("test_agent");
    assert!(metadata.is_ok());
    let meta = metadata.unwrap();
    assert_eq!(meta.id, "test_agent");
}

/// @covers: AgentRegistry trait re-export — metadata error handling
#[test]
fn test_svc_agent_registry_error_metadata_agent_not_found() {
    let registry = TestAgentRegistry::new();
    let result = registry.metadata("nonexistent");
    assert!(result.is_err());
    match result {
        Err(AgentError::NotFound(id)) => assert_eq!(id, "nonexistent"),
        _ => panic!("Expected NotFound error"),
    }
}

/// @covers: AgentRegistry trait re-export — edge case empty registry
#[test]
fn test_svc_agent_registry_edge_is_empty_returns_true() {
    let registry = TestAgentRegistry::new();
    assert!(registry.is_empty());
}
