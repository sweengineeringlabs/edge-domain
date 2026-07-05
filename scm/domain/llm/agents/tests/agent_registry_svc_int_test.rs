#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for AGENT_REGISTRY_SVC constant and AgentRegistry trait re-export.

use async_trait::async_trait;
use edge_domain_observer::StdObserveFactory;
use edge_domain_registry::{
    DeregisterRequest, DeregisterResponse, EmptinessRequest, LenRequest, LenResponse,
    ListIdsRequest, ListIdsResponse, RegisterRequest, RegisterResponse, Registry, RegistryError,
    RegistryLookupRequest, RegistryLookupResponse, TryRegisterRequest, TryRegisterResponse,
};
use edge_llm_agent::{
    Agent, AgentDescriptionRequest, AgentDescriptionResponse, AgentError, AgentIdRequest,
    AgentIdResponse, AgentMetadata, AgentMetadataLookupRequest, AgentMetadataLookupResponse,
    AgentNameRequest, AgentNameResponse, AgentProviderRequest, AgentProviderResponse,
    AgentRegistry, AgentSkillsRequest, AgentSkillsResponse, SkillExecutionRequest,
    SkillExecutionResponse,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
};
use std::sync::{Arc, Mutex};

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
            description: "Test agent for registry testing".to_string(),
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

type AgentEntry = (Arc<dyn Agent>, AgentMetadata);

struct TestAgentRegistry {
    agents: Mutex<std::collections::HashMap<String, AgentEntry>>,
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

    fn register(&self, req: RegisterRequest<Self::Value>) -> Result<RegisterResponse, RegistryError> {
        self.agents.lock().unwrap().insert(
            req.id.clone(),
            (req.entry, create_dummy_metadata(&req.id)),
        );
        Ok(RegisterResponse)
    }

    fn try_register(
        &self,
        req: TryRegisterRequest<Self::Value>,
    ) -> Result<TryRegisterResponse, RegistryError> {
        let mut agents = self.agents.lock().unwrap();
        if agents.contains_key(&req.id) {
            Err(RegistryError::DuplicateId(req.id))
        } else {
            let meta = create_dummy_metadata(&req.id);
            agents.insert(req.id, (req.entry, meta));
            Ok(TryRegisterResponse)
        }
    }

    fn deregister(&self, req: DeregisterRequest) -> Result<DeregisterResponse, RegistryError> {
        let was_present = self.agents.lock().unwrap().remove(&req.id).is_some();
        Ok(DeregisterResponse { was_present })
    }

    fn get(
        &self,
        req: RegistryLookupRequest,
    ) -> Result<RegistryLookupResponse<Self::Value>, RegistryError> {
        let entry = self
            .agents
            .lock()
            .unwrap()
            .get(&req.id)
            .map(|(agent, _)| agent.clone());
        Ok(RegistryLookupResponse { entry })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, RegistryError> {
        let ids = self.agents.lock().unwrap().keys().cloned().collect();
        Ok(ListIdsResponse { ids })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, RegistryError> {
        Ok(LenResponse {
            count: self.agents.lock().unwrap().len(),
        })
    }
}

impl AgentRegistry for TestAgentRegistry {
    fn metadata(
        &self,
        req: AgentMetadataLookupRequest<'_>,
    ) -> Result<AgentMetadataLookupResponse, AgentError> {
        self.agents
            .lock()
            .unwrap()
            .get(req.id)
            .map(|(_, meta)| AgentMetadataLookupResponse {
                metadata: Box::new(meta.clone()),
            })
            .ok_or_else(|| AgentError::NotFound(req.id.to_string()))
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
    registry
        .register(RegisterRequest {
            id: "test".to_string(),
            entry: agent,
        })
        .unwrap();
    let retrieved = registry.get(RegistryLookupRequest {
        id: "test".to_string(),
    });
    let a = retrieved.unwrap().entry.unwrap();
    assert_eq!(a.id(AgentIdRequest).unwrap().id, "test");
}

/// @covers: AgentRegistry trait re-export — inherits Registry::register
#[test]
fn test_svc_agent_registry_happy_register_stores_agent() {
    let registry = TestAgentRegistry::new();
    let agent = Arc::new(TestAgent {
        id: "agent1".to_string(),
    });
    registry
        .register(RegisterRequest {
            id: "agent1".to_string(),
            entry: agent,
        })
        .unwrap();
    let retrieved = registry.get(RegistryLookupRequest {
        id: "agent1".to_string(),
    });
    let a = retrieved.unwrap().entry.unwrap();
    assert_eq!(a.id(AgentIdRequest).unwrap().id, "agent1");
}

/// @covers: AgentRegistry trait re-export — inherits Registry::get
#[test]
fn test_svc_agent_registry_error_get_nonexistent_agent() {
    let registry = TestAgentRegistry::new();
    let result = registry.get(RegistryLookupRequest {
        id: "nonexistent".to_string(),
    });
    assert!(result.unwrap().entry.is_none());
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
    registry
        .register(RegisterRequest {
            id: "agent1".to_string(),
            entry: agent1,
        })
        .unwrap();
    registry
        .register(RegisterRequest {
            id: "agent2".to_string(),
            entry: agent2,
        })
        .unwrap();
    let ids = registry.list_ids(ListIdsRequest).unwrap().ids;
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
    registry
        .register(RegisterRequest {
            id: "test_agent".to_string(),
            entry: agent,
        })
        .unwrap();
    let metadata = registry.metadata(AgentMetadataLookupRequest { id: "test_agent" });
    let meta = metadata.unwrap().metadata;
    assert_eq!(meta.id, "test_agent");
}

/// @covers: AgentRegistry trait re-export — metadata error handling
#[test]
fn test_svc_agent_registry_error_metadata_agent_not_found() {
    let registry = TestAgentRegistry::new();
    let result = registry.metadata(AgentMetadataLookupRequest { id: "nonexistent" });
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
    assert!(registry.is_empty(EmptinessRequest).unwrap().empty);
}
