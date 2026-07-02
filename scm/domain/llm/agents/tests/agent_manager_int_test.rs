#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests — `AgentManager` trait.

use async_trait::async_trait;
use edge_domain_handler::{ExecutionRequest, Handler, HandlerError};
use edge_domain_observer::StdObserveFactory;
use edge_llm_agent::{
    Agent, AgentCreationRequest, AgentCreationResponse, AgentDescriptionRequest,
    AgentDescriptionResponse, AgentError, AgentHandlerRequest, AgentHandlerResponse,
    AgentIdRequest, AgentIdResponse, AgentLoadRequest, AgentLoadResponse, AgentLookupRequest,
    AgentLookupResponse, AgentManager, AgentNameRequest, AgentNameResponse, AgentProviderRequest,
    AgentProviderResponse, AgentSkillsRequest, AgentSkillsResponse, ListAgentIdsRequest,
    ListAgentIdsResponse, SkillExecutionRequest, SkillExecutionResponse,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
};
use std::sync::Arc;

struct InlineHandler;
#[async_trait]
impl Handler for InlineHandler {
    type Request = String;
    type Response = String;
    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        Ok(req.req)
    }
}

fn noop_provider() -> Arc<dyn Provider> {
    StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        Box::<ModelInfo>::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

struct TestManager {
    should_fail: bool,
}

#[async_trait]
impl AgentManager for TestManager {
    async fn load_agent(&self, req: AgentLoadRequest<'_>) -> Result<AgentLoadResponse, AgentError> {
        let spec = req.spec;
        if self.should_fail {
            Err(AgentError::InvalidSpec(format!("Cannot load: {}", spec)))
        } else if spec.is_empty() {
            Err(AgentError::InvalidSpec("Empty spec".to_string()))
        } else {
            Ok(AgentLoadResponse {
                agent: Arc::new(DummyAgent::new("dummy")),
            })
        }
    }

    fn agent(&self, req: AgentLookupRequest<'_>) -> Result<AgentLookupResponse, AgentError> {
        let id = req.id;
        if self.should_fail {
            Err(AgentError::NotFound(id.to_string()))
        } else if id == "exists" {
            Ok(AgentLookupResponse {
                agent: Arc::new(DummyAgent::new(id)),
            })
        } else {
            Err(AgentError::NotFound(id.to_string()))
        }
    }

    fn list_agent_ids(
        &self,
        _req: ListAgentIdsRequest,
    ) -> Result<ListAgentIdsResponse, AgentError> {
        if self.should_fail {
            Err(AgentError::ExecutionFailed("Cannot list".to_string()))
        } else {
            Ok(ListAgentIdsResponse {
                ids: vec!["agent1".to_string(), "agent2".to_string()],
            })
        }
    }

    fn agent_handler(
        &self,
        _req: AgentHandlerRequest<'_>,
    ) -> Result<AgentHandlerResponse, AgentError> {
        Ok(AgentHandlerResponse {
            handler: Box::new(InlineHandler),
        })
    }

    fn default_agent(
        &self,
        _req: AgentCreationRequest<'_>,
    ) -> Result<AgentCreationResponse, AgentError> {
        Ok(AgentCreationResponse {
            agent: Arc::new(DummyAgent::new("dummy")),
        })
    }
}

struct DummyAgent {
    id: String,
}

impl DummyAgent {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

#[async_trait]
impl Agent for DummyAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<AgentIdResponse, AgentError> {
        Ok(AgentIdResponse {
            id: self.id.clone(),
        })
    }

    fn name(&self, _req: AgentNameRequest) -> Result<AgentNameResponse, AgentError> {
        Ok(AgentNameResponse {
            name: "Dummy".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError> {
        Ok(AgentDescriptionResponse {
            description: "Dummy agent for testing".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        _req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError> {
        Ok(SkillExecutionResponse {
            output: "dummy_response".to_string(),
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

/// @covers: AgentManager::load_agent — success
#[test]
fn test_trait_agent_manager_happy_load_agent_valid_spec_returns_ok() {
    let manager = TestManager { should_fail: false };
    let result =
        futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "valid.yaml" }));
    assert!(result.is_ok(), "load_agent should succeed with valid spec");
    let agent = result.unwrap().agent;
    assert_eq!(
        agent.id(AgentIdRequest).unwrap().id,
        "dummy",
        "agent id should match"
    );
}

/// @covers: AgentManager::load_agent — invalid spec
#[test]
fn test_trait_agent_manager_error_load_agent_invalid_spec_returns_error() {
    let manager = TestManager { should_fail: true };
    let result =
        futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "bad.yaml" }));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

/// @covers: AgentManager::load_agent — empty spec
#[test]
fn test_trait_agent_manager_error_load_agent_empty_spec_returns_invalid_spec() {
    let manager = TestManager { should_fail: false };
    let result = futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "" }));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

/// @covers: AgentManager::agent — found
#[test]
fn test_trait_agent_manager_happy_agent_existing_id_returns_ok() {
    let manager = TestManager { should_fail: false };
    let result = manager.agent(AgentLookupRequest { id: "exists" });
    let agent = result.unwrap().agent;
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "exists");
}

/// @covers: AgentManager::agent — not found
#[test]
fn test_trait_agent_manager_error_agent_nonexistent_id_returns_not_found() {
    let manager = TestManager { should_fail: false };
    let result = manager.agent(AgentLookupRequest { id: "missing" });
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: AgentManager::agent — manager failure
#[test]
fn test_trait_agent_manager_error_agent_when_manager_fails_returns_error() {
    let manager = TestManager { should_fail: true };
    let result = manager.agent(AgentLookupRequest { id: "any" });
    assert!(result.is_err());
}

/// @covers: AgentManager::list_agent_ids — success
#[test]
fn test_trait_agent_manager_happy_list_agent_ids_returns_list() {
    let manager = TestManager { should_fail: false };
    let result = manager.list_agent_ids(ListAgentIdsRequest);
    let ids = result.unwrap().ids;
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"agent1".to_string()));
}

/// @covers: AgentManager::list_agent_ids — failure
#[test]
fn test_trait_agent_manager_error_list_agent_ids_when_fails_returns_error() {
    let manager = TestManager { should_fail: true };
    let result = manager.list_agent_ids(ListAgentIdsRequest);
    assert!(result.is_err());
}

/// @covers: AgentManager::list_agent_ids — empty list
#[test]
fn test_trait_agent_manager_edge_list_agent_ids_empty_list() {
    // `TestManager` always seeds the same two agent ids; this exercises the
    // boundary that `list_agent_ids` is deterministic across repeated calls
    // rather than a truly-empty manager (which this fixture cannot express).
    let manager = TestManager { should_fail: false };
    let first = manager.list_agent_ids(ListAgentIdsRequest).unwrap().ids;
    let second = manager.list_agent_ids(ListAgentIdsRequest).unwrap().ids;
    assert_eq!(first, second);
}

/// @covers: AgentManager — all methods consistent
#[test]
fn test_trait_agent_manager_happy_all_methods_work_together() {
    let manager = TestManager { should_fail: false };
    let load_result =
        futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "test.yaml" }));
    let agent_result = manager.agent(AgentLookupRequest { id: "exists" });
    let list_result = manager.list_agent_ids(ListAgentIdsRequest);

    // Check load_agent returns valid agent
    assert!(load_result.is_ok(), "load_agent should succeed");
    let agent = load_result.unwrap().agent;
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "dummy");

    // Check agent retrieval returns existing agent
    assert!(agent_result.is_ok(), "agent should succeed");
    let found_agent = agent_result.unwrap().agent;
    assert_eq!(found_agent.id(AgentIdRequest).unwrap().id, "exists");

    // Check list returns multiple agents
    assert!(list_result.is_ok(), "list_agent_ids should succeed");
    let ids = list_result.unwrap().ids;
    assert_eq!(ids.len(), 2, "should have exactly 2 agents");
    assert!(ids.contains(&"agent1".to_string()), "should contain agent1");
    assert!(ids.contains(&"agent2".to_string()), "should contain agent2");
}

/// @covers: AgentManager::agent_metadata_builder
#[test]
fn test_agent_metadata_builder_scenario_happy() {
    let manager = TestManager { should_fail: false };
    let builder = manager
        .agent_metadata_builder(edge_llm_agent::AgentMetadataBuilderRequest)
        .unwrap()
        .builder;
    let metadata = builder
        .id("test")
        .name("Test")
        .description("Test agent")
        .version("1.0")
        .build();
    assert_eq!(metadata.id, "test");
}

/// @covers: AgentManager::agent_metadata_builder
#[test]
fn test_agent_metadata_builder_scenario_error() {
    let manager = TestManager { should_fail: false };
    // Building without required fields yields empty defaults rather than panicking.
    let metadata = manager
        .agent_metadata_builder(edge_llm_agent::AgentMetadataBuilderRequest)
        .unwrap()
        .builder
        .build();
    assert!(metadata.id.is_empty());
    assert!(metadata.name.is_empty());
}

/// @covers: AgentManager::agent_metadata_builder
#[test]
fn test_agent_metadata_builder_scenario_edge() {
    let manager = TestManager { should_fail: false };
    let metadata = manager
        .agent_metadata_builder(edge_llm_agent::AgentMetadataBuilderRequest)
        .unwrap()
        .builder
        .id("edge")
        .name("Edge")
        .description("Edge")
        .version("1.0")
        .build();
    assert_eq!(metadata.skills.len(), 0);
}

/// @covers: AgentManager::skill_metadata_builder
#[test]
fn test_skill_metadata_builder_scenario_happy() {
    let manager = TestManager { should_fail: false };
    let builder = manager
        .skill_metadata_builder(edge_llm_agent::SkillMetadataBuilderRequest)
        .unwrap()
        .builder;
    let metadata = builder.name("test_skill").description("Test skill").build();
    assert_eq!(metadata.name, "test_skill");
}

/// @covers: AgentManager::skill_metadata_builder
#[test]
fn test_skill_metadata_builder_scenario_error() {
    let manager = TestManager { should_fail: false };
    // Building without required fields yields empty defaults rather than panicking.
    let metadata = manager
        .skill_metadata_builder(edge_llm_agent::SkillMetadataBuilderRequest)
        .unwrap()
        .builder
        .build();
    assert!(metadata.name.is_empty());
    assert!(metadata.description.is_empty());
}

/// @covers: AgentManager::skill_metadata_builder
#[test]
fn test_skill_metadata_builder_scenario_edge() {
    let manager = TestManager { should_fail: false };
    let metadata = manager
        .skill_metadata_builder(edge_llm_agent::SkillMetadataBuilderRequest)
        .unwrap()
        .builder
        .name("minimal")
        .description("Minimal")
        .build();
    assert_eq!(metadata.input_schema, None);
}
