#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests — `AgentRegistry` trait.

use edge_domain_observer::StdObserveFactory;
use edge_domain_registry::{
    DeregisterRequest, DeregisterResponse, EmptinessRequest, LenRequest, LenResponse,
    ListIdsRequest, ListIdsResponse, RegisterRequest, RegisterResponse, Registry, RegistryError,
    RegistryLookupRequest, RegistryLookupResponse, TryRegisterRequest, TryRegisterResponse,
};
use edge_llm_agent::{
    Agent, AgentDescriptionRequest, AgentDescriptionResponse, AgentError, AgentIdRequest,
    AgentIdResponse, AgentMetadata, AgentMetadataLookupRequest, AgentNameRequest,
    AgentNameResponse, AgentProviderRequest, AgentProviderResponse, AgentRegistry,
    AgentSkillsRequest, AgentSkillsResponse, SkillExecutionRequest, SkillExecutionResponse,
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

struct DummyAgent;

#[async_trait::async_trait]
impl Agent for DummyAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
        Ok(AgentIdResponse {
            id: "test".to_string(),
        })
    }

    fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
        Ok(AgentNameResponse {
            name: "Test".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError> {
        Ok(AgentDescriptionResponse {
            description: "Test agent".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        _req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError> {
        Ok(SkillExecutionResponse {
            output: "ok".to_string(),
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

struct TestRegistry {
    has_agent: bool,
}

impl Registry for TestRegistry {
    type Value = dyn Agent;

    fn register(&self, _req: RegisterRequest<Self::Value>) -> Result<RegisterResponse, RegistryError> {
        Ok(RegisterResponse)
    }

    fn try_register(
        &self,
        _req: TryRegisterRequest<Self::Value>,
    ) -> Result<TryRegisterResponse, RegistryError> {
        Ok(TryRegisterResponse)
    }

    fn deregister(&self, _req: DeregisterRequest) -> Result<DeregisterResponse, RegistryError> {
        Ok(DeregisterResponse { was_present: true })
    }

    fn get(
        &self,
        req: RegistryLookupRequest,
    ) -> Result<RegistryLookupResponse<Self::Value>, RegistryError> {
        let entry = if self.has_agent && req.id == "found" {
            Some(Arc::new(DummyAgent) as Arc<dyn Agent>)
        } else {
            None
        };
        Ok(RegistryLookupResponse { entry })
    }

    fn list_ids(&self, _req: ListIdsRequest) -> Result<ListIdsResponse, RegistryError> {
        let ids = if self.has_agent {
            vec!["agent1".to_string()]
        } else {
            vec![]
        };
        Ok(ListIdsResponse { ids })
    }

    fn len(&self, _req: LenRequest) -> Result<LenResponse, RegistryError> {
        let count = if self.has_agent { 1 } else { 0 };
        Ok(LenResponse { count })
    }
}

impl AgentRegistry for TestRegistry {
    fn metadata(
        &self,
        req: AgentMetadataLookupRequest<'_>,
    ) -> Result<edge_llm_agent::AgentMetadataLookupResponse, AgentError> {
        let id = req.id;
        if self.has_agent && id == "found" {
            Ok(edge_llm_agent::AgentMetadataLookupResponse {
                metadata: Box::new(AgentMetadata {
                    id: id.to_string(),
                    name: "Found Agent".to_string(),
                    description: "Agent found in registry".to_string(),
                    version: "1.0.0".to_string(),
                    skills: vec![],
                    patterns: vec!["react".to_string()],
                }),
            })
        } else {
            Err(AgentError::NotFound(id.to_string()))
        }
    }
}

/// @covers: AgentRegistry::metadata — found
#[test]
fn test_trait_agent_registry_happy_metadata_existing_agent_returns_ok() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.metadata(AgentMetadataLookupRequest { id: "found" });
    let meta = result.unwrap().metadata;
    assert_eq!(meta.id, "found");
    assert_eq!(meta.name, "Found Agent");
}

/// @covers: AgentRegistry::metadata — not found
#[test]
fn test_trait_agent_registry_error_metadata_missing_agent_returns_not_found() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.metadata(AgentMetadataLookupRequest { id: "missing" });
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: Registry::get (inherited)
#[test]
fn test_trait_agent_registry_happy_get_existing_returns_some() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.get(RegistryLookupRequest {
        id: "found".to_string(),
    });
    let agent = result.unwrap().entry.unwrap();
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "test");
}

/// @covers: Registry::get (inherited) — missing
#[test]
fn test_trait_agent_registry_error_get_missing_returns_none() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.get(RegistryLookupRequest {
        id: "missing".to_string(),
    });
    assert!(result.unwrap().entry.is_none());
}

/// @covers: Registry::list_ids (inherited)
#[test]
fn test_trait_agent_registry_happy_list_ids_returns_ids() {
    let registry = TestRegistry { has_agent: true };
    let ids = registry.list_ids(ListIdsRequest).unwrap().ids;
    assert_eq!(ids.len(), 1);
    assert!(ids.contains(&"agent1".to_string()));
}

/// @covers: Registry::list_ids (inherited) — empty
#[test]
fn test_trait_agent_registry_edge_list_ids_empty_returns_empty_vec() {
    let registry = TestRegistry { has_agent: false };
    let ids = registry.list_ids(ListIdsRequest).unwrap().ids;
    assert_eq!(ids.len(), 0);
}

/// @covers: Registry::len (inherited)
#[test]
fn test_trait_agent_registry_happy_len_with_agent_returns_one() {
    let registry = TestRegistry { has_agent: true };
    assert_eq!(registry.len(LenRequest).unwrap().count, 1);
}

/// @covers: Registry::len (inherited) — empty
#[test]
fn test_trait_agent_registry_happy_len_empty_returns_zero() {
    let registry = TestRegistry { has_agent: false };
    assert_eq!(registry.len(LenRequest).unwrap().count, 0);
}

/// @covers: Registry::is_empty (inherited)
#[test]
fn test_trait_agent_registry_happy_is_empty_when_empty_returns_true() {
    let registry = TestRegistry { has_agent: false };
    assert!(registry.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: Registry::is_empty (inherited) — not empty
#[test]
fn test_trait_agent_registry_happy_is_empty_when_has_items_returns_false() {
    let registry = TestRegistry { has_agent: true };
    assert!(!registry.is_empty(EmptinessRequest).unwrap().empty);
}

/// @covers: AgentRegistry — metadata + Registry methods work together
#[test]
fn test_trait_agent_registry_happy_all_methods_together() {
    let registry = TestRegistry { has_agent: true };
    assert!(!registry.is_empty(EmptinessRequest).unwrap().empty);
    let meta = registry
        .metadata(AgentMetadataLookupRequest { id: "found" })
        .unwrap()
        .metadata;
    assert_eq!(meta.id, "found");
    let agent = registry
        .get(RegistryLookupRequest {
            id: "found".to_string(),
        })
        .unwrap()
        .entry
        .unwrap();
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "test");
    assert_eq!(registry.list_ids(ListIdsRequest).unwrap().ids.len(), 1);
}
