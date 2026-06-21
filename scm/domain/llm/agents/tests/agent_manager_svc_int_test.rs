#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_MANAGER_SVC constant and AgentManager trait re-export.

use async_trait::async_trait;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_llm_agent::{Agent, AgentError, AgentManager, NoopAgentManager, Skill};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderConfig, ProviderBootstrap, StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    Arc::new(StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
    ))
}

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
        "Test agent for manager testing"
    }

    async fn execute_skill(&self, _skill_name: &str, _input: String) -> Result<String, AgentError> {
        Ok("result".to_string())
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

struct TestAgentManager {
    agents: Vec<Arc<dyn Agent>>,
}

#[async_trait]
impl AgentManager for TestAgentManager {
    async fn load_agent(&self, spec: &str) -> Result<Arc<dyn Agent>, AgentError> {
        if spec == "valid" {
            Ok(Arc::new(TestAgent {
                id: "loaded_agent".to_string(),
            }))
        } else {
            Err(AgentError::InvalidSpec(spec.to_string()))
        }
    }

    fn agent(&self, id: &str) -> Result<Arc<dyn Agent>, AgentError> {
        self.agents
            .iter()
            .find(|a| a.id() == id)
            .cloned()
            .ok_or_else(|| AgentError::NotFound(id.to_string()))
    }

    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError> {
        Ok(self.agents.iter().map(|a| a.id().to_string()).collect())
    }

    fn agent_handler(&self, skill: &str) -> Box<dyn Handler<Request = String, Response = String>> {
        NoopAgentManager.agent_handler(skill)
    }

    fn default_agent(
        &self,
        id: &str,
        name: &str,
        description: &str,
        provider: Arc<dyn Provider>,
        skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
    ) -> Arc<dyn Agent> {
        NoopAgentManager.default_agent(id, name, description, provider, skills)
    }
}

/// @covers: AGENT_MANAGER_SVC constant
#[test]
fn test_svc_agent_manager_svc_happy_constant_equals_agent_manager() {
    assert_eq!(edge_llm_agent::AGENT_MANAGER_SVC, "agent_manager");
}

/// @covers: AGENT_MANAGER_SVC constant
#[test]
fn test_svc_agent_manager_svc_error_constant_not_empty() {
    assert!(!edge_llm_agent::AGENT_MANAGER_SVC.is_empty());
}

/// @covers: AGENT_MANAGER_SVC constant
#[test]
fn test_svc_agent_manager_svc_edge_constant_is_valid_identifier() {
    let svc = edge_llm_agent::AGENT_MANAGER_SVC;
    assert!(svc.chars().all(|c| c.is_ascii_lowercase() || c == '_'));
}

/// @covers: AgentManager trait re-export
#[test]
fn test_svc_agent_manager_happy_trait_can_be_implemented() {
    let manager = TestAgentManager {
        agents: vec![Arc::new(TestAgent {
            id: "test".to_string(),
        })],
    };
    let agent = manager.agent("test");
    assert!(agent.is_ok());
}

/// @covers: AgentManager trait re-export — load_agent
#[test]
fn test_svc_agent_manager_happy_load_agent_valid_spec() {
    let manager = TestAgentManager { agents: vec![] };
    let result = futures::executor::block_on(manager.load_agent("valid"));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id(), "loaded_agent");
}

/// @covers: AgentManager trait re-export — load_agent error
#[test]
fn test_svc_agent_manager_error_load_agent_invalid_spec() {
    let manager = TestAgentManager { agents: vec![] };
    let result = futures::executor::block_on(manager.load_agent("invalid"));
    assert!(result.is_err());
    match result {
        Err(AgentError::InvalidSpec(spec)) => assert_eq!(spec, "invalid"),
        _ => panic!("Expected InvalidSpec error"),
    }
}

/// @covers: AgentManager trait re-export — agent lookup
#[test]
fn test_svc_agent_manager_error_agent_not_found() {
    let manager = TestAgentManager { agents: vec![] };
    let result = manager.agent("nonexistent");
    assert!(result.is_err());
}

/// @covers: AgentManager trait re-export — list_agent_ids
#[test]
fn test_svc_agent_manager_happy_list_agent_ids_returns_list() {
    let manager = TestAgentManager {
        agents: vec![Arc::new(TestAgent {
            id: "agent1".to_string(),
        })],
    };
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
    let ids = result.unwrap();
    assert_eq!(ids.len(), 1);
    assert_eq!(ids[0], "agent1");
}

/// @covers: AgentManager trait re-export — list_agent_ids empty
#[test]
fn test_svc_agent_manager_edge_list_agent_ids_empty() {
    let manager = TestAgentManager { agents: vec![] };
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}
