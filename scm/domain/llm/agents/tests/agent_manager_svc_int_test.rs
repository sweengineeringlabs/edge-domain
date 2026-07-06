#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_MANAGER_SVC constant and AgentManager trait re-export.

use async_trait::async_trait;
use edge_domain_observer::StdObserveFactory;
use edge_llm_agent::{
    Agent, AgentCreationRequest, AgentCreationResponse, AgentDescriptionRequest,
    AgentDescriptionResponse, AgentError, AgentHandlerRequest, AgentHandlerResponse,
    AgentIdRequest, AgentIdResponse, AgentLoadRequest, AgentLoadResponse, AgentLookupRequest,
    AgentLookupResponse, AgentManager, AgentNameRequest, AgentNameResponse, AgentProviderRequest,
    AgentProviderResponse, AgentSkillsRequest, AgentSkillsResponse, ListAgentIdsRequest,
    ListAgentIdsResponse, NoopAgentManager, SkillExecutionRequest, SkillExecutionResponse,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        Box::<ModelInfo>::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

struct TestAgent {
    id: String,
}

#[async_trait]
impl Agent for TestAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
        Ok(AgentIdResponse {
            id: self.id.clone(),
        })
    }

    fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
        Ok(AgentNameResponse {
            name: "Test Agent".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError> {
        Ok(AgentDescriptionResponse {
            description: "Test agent for manager testing".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        _req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError> {
        Ok(SkillExecutionResponse {
            output: "result".to_string(),
        })
    }

    fn skills(&self, _req: AgentSkillsRequest) -> Result<AgentSkillsResponse, AgentError> {
        Ok(AgentSkillsResponse { skills: vec![] })
    }

    fn provider(&self, _req: AgentProviderRequest) -> Result<AgentProviderResponse, AgentError> {
        Ok(AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

struct TestAgentManager {
    agents: Vec<Arc<dyn Agent>>,
}

#[async_trait]
impl AgentManager for TestAgentManager {
    async fn load_agent(&self, req: AgentLoadRequest<'_>) -> Result<AgentLoadResponse, AgentError> {
        let spec = req.spec;
        if spec == "valid" {
            Ok(AgentLoadResponse {
                agent: Arc::new(TestAgent {
                    id: "loaded_agent".to_string(),
                }),
            })
        } else {
            Err(AgentError::InvalidSpec(spec.to_string()))
        }
    }

    fn agent(&self, req: AgentLookupRequest<'_>) -> Result<AgentLookupResponse, AgentError> {
        let id = req.id;
        self.agents
            .iter()
            .find(|a| a.id(AgentIdRequest).map(|r| r.id).unwrap_or_default() == id)
            .cloned()
            .map(|agent| AgentLookupResponse { agent })
            .ok_or_else(|| AgentError::NotFound(id.to_string()))
    }

    fn list_agent_ids(
        &self,
        _req: ListAgentIdsRequest,
    ) -> Result<ListAgentIdsResponse, AgentError> {
        Ok(ListAgentIdsResponse {
            ids: self
                .agents
                .iter()
                .map(|a| a.id(AgentIdRequest).map(|r| r.id).unwrap_or_default())
                .collect(),
        })
    }

    fn agent_handler(
        &self,
        req: AgentHandlerRequest<'_>,
    ) -> Result<AgentHandlerResponse, AgentError> {
        NoopAgentManager.agent_handler(req)
    }

    fn default_agent(
        &self,
        req: AgentCreationRequest<'_>,
    ) -> Result<AgentCreationResponse, AgentError> {
        NoopAgentManager.default_agent(req)
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
    let agent = manager.agent(AgentLookupRequest { id: "test" });
    let a = agent.unwrap().agent;
    assert_eq!(a.id(AgentIdRequest).unwrap().id, "test");
}

/// @covers: AgentManager trait re-export — load_agent
#[test]
fn test_svc_agent_manager_happy_load_agent_valid_spec() {
    let manager = TestAgentManager { agents: vec![] };
    let result =
        futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "valid" }));
    let agent = result.unwrap().agent;
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "loaded_agent");
}

/// @covers: AgentManager trait re-export — load_agent error
#[test]
fn test_svc_agent_manager_error_load_agent_invalid_spec() {
    let manager = TestAgentManager { agents: vec![] };
    let result =
        futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "invalid" }));
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
    let result = manager.agent(AgentLookupRequest { id: "nonexistent" });
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
    let result = manager.list_agent_ids(ListAgentIdsRequest);
    let ids = result.unwrap().ids;
    assert_eq!(ids.len(), 1);
    assert_eq!(ids[0], "agent1");
}

/// @covers: AgentManager trait re-export — list_agent_ids empty
#[test]
fn test_svc_agent_manager_edge_list_agent_ids_empty() {
    let manager = TestAgentManager { agents: vec![] };
    let result = manager.list_agent_ids(ListAgentIdsRequest);
    let ids = result.unwrap().ids;
    assert_eq!(ids.len(), 0);
}
