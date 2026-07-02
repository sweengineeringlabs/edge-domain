#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Comprehensive trait method scenario coverage tests.
//!
//! Tests all public trait methods in Agent, Skill, AgentManager, and AgentRegistry
//! with happy, error, and edge cases.

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerContext, HandlerError, IdRequest, IdResponse,
};
use edge_domain_observer::StdObserveFactory;
use edge_domain_registry::Registry;
use edge_domain_security::{SecurityBootstrap, SecurityServices};
use edge_llm_agent::{
    Agent, AgentCreationRequest, AgentDescriptionRequest, AgentError, AgentHandlerRequest,
    AgentIdRequest, AgentLoadRequest, AgentLookupRequest, AgentManager, AgentMetadata,
    AgentMetadataLookupRequest, AgentNameRequest, AgentProviderRequest, AgentRegistry,
    AgentSkillsRequest, ListAgentIdsRequest, NoopAgentManager, Parameter, Skill,
    SkillDescriptionRequest, SkillExecutionRequest, SkillLookupRequest, SkillMetadata,
    SkillMetadataLookupRequest, SkillNameRequest, SkillParametersRequest,
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

// ============================================================================
// Test Fixtures
// ============================================================================

/// A test agent that succeeds
struct SuccessAgent;

#[async_trait]
impl Agent for SuccessAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<edge_llm_agent::AgentIdResponse, AgentError> {
        Ok(edge_llm_agent::AgentIdResponse {
            id: "success_agent".to_string(),
        })
    }

    fn name(
        &self,
        _req: AgentNameRequest,
    ) -> Result<edge_llm_agent::AgentNameResponse, AgentError> {
        Ok(edge_llm_agent::AgentNameResponse {
            name: "Success Agent".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<edge_llm_agent::AgentDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::AgentDescriptionResponse {
            description: "Test agent that succeeds".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<edge_llm_agent::SkillExecutionResponse, AgentError> {
        Ok(edge_llm_agent::SkillExecutionResponse {
            output: format!("{}:{}", req.skill_name, req.input),
        })
    }

    fn skills(
        &self,
        _req: AgentSkillsRequest,
    ) -> Result<edge_llm_agent::AgentSkillsResponse, AgentError> {
        Ok(edge_llm_agent::AgentSkillsResponse {
            skills: vec![Arc::new(TestSkill) as Arc<dyn Skill<Request = String, Response = String>>],
        })
    }

    fn provider(
        &self,
        _req: AgentProviderRequest,
    ) -> Result<edge_llm_agent::AgentProviderResponse, AgentError> {
        Ok(edge_llm_agent::AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

/// A test agent that fails
struct FailingAgent;

#[async_trait]
impl Agent for FailingAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<edge_llm_agent::AgentIdResponse, AgentError> {
        Ok(edge_llm_agent::AgentIdResponse {
            id: "failing_agent".to_string(),
        })
    }

    fn name(
        &self,
        _req: AgentNameRequest,
    ) -> Result<edge_llm_agent::AgentNameResponse, AgentError> {
        Ok(edge_llm_agent::AgentNameResponse {
            name: "Failing Agent".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<edge_llm_agent::AgentDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::AgentDescriptionResponse {
            description: "Test agent that fails".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        _req: SkillExecutionRequest<'_>,
    ) -> Result<edge_llm_agent::SkillExecutionResponse, AgentError> {
        Err(AgentError::ExecutionFailed(
            "intentional failure".to_string(),
        ))
    }

    fn skills(
        &self,
        _req: AgentSkillsRequest,
    ) -> Result<edge_llm_agent::AgentSkillsResponse, AgentError> {
        Ok(edge_llm_agent::AgentSkillsResponse { skills: vec![] })
    }

    fn provider(
        &self,
        _req: AgentProviderRequest,
    ) -> Result<edge_llm_agent::AgentProviderResponse, AgentError> {
        Ok(edge_llm_agent::AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

/// A test agent with empty fields
struct EmptyAgent;

#[async_trait]
impl Agent for EmptyAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<edge_llm_agent::AgentIdResponse, AgentError> {
        Ok(edge_llm_agent::AgentIdResponse { id: String::new() })
    }

    fn name(
        &self,
        _req: AgentNameRequest,
    ) -> Result<edge_llm_agent::AgentNameResponse, AgentError> {
        Ok(edge_llm_agent::AgentNameResponse {
            name: String::new(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<edge_llm_agent::AgentDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::AgentDescriptionResponse {
            description: String::new(),
        })
    }

    async fn execute_skill(
        &self,
        _req: SkillExecutionRequest<'_>,
    ) -> Result<edge_llm_agent::SkillExecutionResponse, AgentError> {
        Ok(edge_llm_agent::SkillExecutionResponse {
            output: String::new(),
        })
    }

    fn skills(
        &self,
        _req: AgentSkillsRequest,
    ) -> Result<edge_llm_agent::AgentSkillsResponse, AgentError> {
        Ok(edge_llm_agent::AgentSkillsResponse { skills: vec![] })
    }

    fn provider(
        &self,
        _req: AgentProviderRequest,
    ) -> Result<edge_llm_agent::AgentProviderResponse, AgentError> {
        Ok(edge_llm_agent::AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

/// A test skill implementation
struct TestSkill;

#[async_trait]
impl Handler for TestSkill {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "test_skill".to_string(),
        })
    }

    async fn execute(&self, req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        Ok(format!("skill_response:{}", req.req))
    }
}

impl Skill for TestSkill {
    fn name(
        &self,
        _req: SkillNameRequest,
    ) -> Result<edge_llm_agent::SkillNameResponse, AgentError> {
        Ok(edge_llm_agent::SkillNameResponse {
            name: "test_skill_name".to_string(),
        })
    }

    fn description(
        &self,
        _req: SkillDescriptionRequest,
    ) -> Result<edge_llm_agent::SkillDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::SkillDescriptionResponse {
            description: "Test skill for integration tests".to_string(),
        })
    }

    fn parameters(
        &self,
        _req: SkillParametersRequest,
    ) -> Result<edge_llm_agent::SkillParametersResponse, AgentError> {
        Ok(edge_llm_agent::SkillParametersResponse {
            parameters: vec![
                Parameter {
                    name: "input".to_string(),
                    description: "Input parameter".to_string(),
                    param_type: "string".to_string(),
                    required: true,
                },
                Parameter {
                    name: "optional".to_string(),
                    description: "Optional parameter".to_string(),
                    param_type: "number".to_string(),
                    required: false,
                },
            ],
        })
    }

    fn metadata(
        &self,
        _req: SkillMetadataLookupRequest,
    ) -> Result<edge_llm_agent::SkillMetadataLookupResponse, AgentError> {
        Ok(edge_llm_agent::SkillMetadataLookupResponse {
            metadata: Box::new(SkillMetadata {
                name: self.name(SkillNameRequest)?.name,
                description: self.description(SkillDescriptionRequest)?.description,
                input_schema: Some(r#"{"type":"string"}"#.to_string()),
                output_schema: Some(r#"{"type":"string"}"#.to_string()),
                async_execution: true,
                long_running: false,
            }),
        })
    }
}

/// A minimal skill with no parameters
struct MinimalSkill;

#[async_trait]
impl Handler for MinimalSkill {
    type Request = String;
    type Response = String;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: "minimal".to_string(),
        })
    }

    async fn execute(&self, _req: ExecutionRequest<'_, String>) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("minimal error".to_string()))
    }
}

impl Skill for MinimalSkill {
    fn name(
        &self,
        _req: SkillNameRequest,
    ) -> Result<edge_llm_agent::SkillNameResponse, AgentError> {
        Ok(edge_llm_agent::SkillNameResponse {
            name: "minimal".to_string(),
        })
    }

    fn description(
        &self,
        _req: SkillDescriptionRequest,
    ) -> Result<edge_llm_agent::SkillDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::SkillDescriptionResponse {
            description: String::new(),
        })
    }
}

/// A test agent manager with configurable behavior
struct TestAgentManager {
    agents: std::collections::HashMap<String, Arc<dyn Agent>>,
    fail_load: bool,
}

impl TestAgentManager {
    fn new() -> Self {
        let mut agents = std::collections::HashMap::new();
        agents.insert(
            "success_agent".to_string(),
            Arc::new(SuccessAgent) as Arc<dyn Agent>,
        );
        agents.insert(
            "failing_agent".to_string(),
            Arc::new(FailingAgent) as Arc<dyn Agent>,
        );

        TestAgentManager {
            agents,
            fail_load: false,
        }
    }

    fn with_failure(mut self, fail: bool) -> Self {
        self.fail_load = fail;
        self
    }
}

#[async_trait]
impl AgentManager for TestAgentManager {
    async fn load_agent(
        &self,
        req: AgentLoadRequest<'_>,
    ) -> Result<edge_llm_agent::AgentLoadResponse, AgentError> {
        let spec = req.spec;
        if self.fail_load {
            return Err(AgentError::InvalidSpec(format!("Failed to load: {}", spec)));
        }
        if spec.is_empty() {
            return Err(AgentError::InvalidSpec("Empty spec".to_string()));
        }
        if spec == "success.yaml" {
            Ok(edge_llm_agent::AgentLoadResponse {
                agent: Arc::new(SuccessAgent),
            })
        } else {
            Err(AgentError::InvalidSpec(format!("Unknown spec: {}", spec)))
        }
    }

    fn agent(
        &self,
        req: AgentLookupRequest<'_>,
    ) -> Result<edge_llm_agent::AgentLookupResponse, AgentError> {
        self.agents
            .get(req.id)
            .cloned()
            .map(|agent| edge_llm_agent::AgentLookupResponse { agent })
            .ok_or_else(|| AgentError::NotFound(req.id.to_string()))
    }

    fn list_agent_ids(
        &self,
        _req: ListAgentIdsRequest,
    ) -> Result<edge_llm_agent::ListAgentIdsResponse, AgentError> {
        Ok(edge_llm_agent::ListAgentIdsResponse {
            ids: self.agents.keys().cloned().collect(),
        })
    }

    fn agent_handler(
        &self,
        req: AgentHandlerRequest<'_>,
    ) -> Result<edge_llm_agent::AgentHandlerResponse, AgentError> {
        NoopAgentManager.agent_handler(req)
    }

    fn default_agent(
        &self,
        req: AgentCreationRequest<'_>,
    ) -> Result<edge_llm_agent::AgentCreationResponse, AgentError> {
        NoopAgentManager.default_agent(req)
    }
}

/// A test agent registry with configurable behavior
struct TestAgentRegistry {
    agents: std::collections::HashMap<String, Arc<dyn Agent>>,
}

impl TestAgentRegistry {
    fn new() -> Self {
        let mut agents = std::collections::HashMap::new();
        agents.insert(
            "test_agent".to_string(),
            Arc::new(SuccessAgent) as Arc<dyn Agent>,
        );
        agents.insert(
            "fail_agent".to_string(),
            Arc::new(FailingAgent) as Arc<dyn Agent>,
        );

        TestAgentRegistry { agents }
    }

    fn empty() -> Self {
        TestAgentRegistry {
            agents: std::collections::HashMap::new(),
        }
    }
}

impl Registry for TestAgentRegistry {
    type Value = dyn Agent;

    fn register(&self, _id: &str, _entry: Arc<Self::Value>) {}

    fn try_register(
        &self,
        _id: &str,
        _entry: Arc<Self::Value>,
    ) -> Result<(), edge_domain_registry::RegistryError> {
        Ok(())
    }

    fn deregister(&self, _id: &str) -> bool {
        true
    }

    fn get(&self, id: &str) -> Option<Arc<Self::Value>> {
        self.agents.get(id).cloned()
    }

    fn list_ids(&self) -> Vec<String> {
        self.agents.keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.agents.len()
    }
}

impl AgentRegistry for TestAgentRegistry {
    fn metadata(
        &self,
        req: AgentMetadataLookupRequest<'_>,
    ) -> Result<edge_llm_agent::AgentMetadataLookupResponse, AgentError> {
        if let Some(agent) = self.agents.get(req.id) {
            Ok(edge_llm_agent::AgentMetadataLookupResponse {
                metadata: Box::new(AgentMetadata {
                    id: req.id.to_string(),
                    name: agent.name(AgentNameRequest)?.name,
                    description: agent.description(AgentDescriptionRequest)?.description,
                    version: "1.0.0".to_string(),
                    skills: vec![],
                    patterns: vec!["test".to_string()],
                }),
            })
        } else {
            Err(AgentError::NotFound(req.id.to_string()))
        }
    }
}

// ============================================================================
// Agent::id Tests
// ============================================================================

/// @covers: Agent::id
#[test]
fn test_id_agent_happy() {
    assert_eq!(SuccessAgent.id(AgentIdRequest).unwrap().id, "success_agent");
}

/// @covers: Agent::id
#[test]
fn test_id_agent_error() {
    let agent = EmptyAgent;
    assert_eq!(agent.id(AgentIdRequest).unwrap().id, "");
}

/// @covers: Agent::id
#[test]
fn test_id_agent_edge() {
    let agent1 = SuccessAgent;
    let agent2 = FailingAgent;
    assert_ne!(
        agent1.id(AgentIdRequest).unwrap().id,
        agent2.id(AgentIdRequest).unwrap().id
    );
}

// ============================================================================
// Agent::name Tests
// ============================================================================

/// @covers: Agent::name
#[test]
fn test_name_agent_happy() {
    assert_eq!(
        SuccessAgent.name(AgentNameRequest).unwrap().name,
        "Success Agent"
    );
}

/// @covers: Agent::name
#[test]
fn test_name_agent_error() {
    assert_eq!(EmptyAgent.name(AgentNameRequest).unwrap().name, "");
}

/// @covers: Agent::name
#[test]
fn test_name_agent_edge() {
    let name = SuccessAgent.name(AgentNameRequest).unwrap().name;
    assert!(!name.is_empty());
    assert!(name.contains("Agent"));
}

// ============================================================================
// Agent::description Tests
// ============================================================================

/// @covers: Agent::description
#[test]
fn test_description_agent_happy() {
    let desc = SuccessAgent
        .description(AgentDescriptionRequest)
        .unwrap()
        .description;
    assert_eq!(desc, "Test agent that succeeds");
}

/// @covers: Agent::description
#[test]
fn test_description_agent_error() {
    assert_eq!(
        EmptyAgent
            .description(AgentDescriptionRequest)
            .unwrap()
            .description,
        ""
    );
}

/// @covers: Agent::description
#[test]
fn test_description_agent_edge() {
    let success_desc = SuccessAgent
        .description(AgentDescriptionRequest)
        .unwrap()
        .description;
    let failing_desc = FailingAgent
        .description(AgentDescriptionRequest)
        .unwrap()
        .description;
    assert_ne!(success_desc, failing_desc);
}

// ============================================================================
// Agent::execute_skill Tests
// ============================================================================

/// @covers: Agent::execute_skill
#[test]
fn test_execute_skill_agent_happy() {
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(SuccessAgent.execute_skill(SkillExecutionRequest {
        skill_name: "code_review",
        input: "input.rs".to_string(),
        ctx,
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().output, "code_review:input.rs");
}

/// @covers: Agent::execute_skill
#[test]
fn test_execute_skill_agent_error() {
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(FailingAgent.execute_skill(SkillExecutionRequest {
        skill_name: "any_skill",
        input: "input".to_string(),
        ctx,
    }));
    assert!(result.is_err());
    match result {
        Err(AgentError::ExecutionFailed(msg)) => {
            assert_eq!(msg, "intentional failure");
        }
        _ => panic!("Expected ExecutionFailed"),
    }
}

/// @covers: Agent::execute_skill
#[test]
fn test_execute_skill_agent_edge() {
    let security = SecurityServices::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(SuccessAgent.execute_skill(SkillExecutionRequest {
        skill_name: "",
        input: "".to_string(),
        ctx,
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().output, ":");
}

// ============================================================================
// Agent::skills Tests
// ============================================================================

/// @covers: Agent::skills
#[test]
fn test_skills_agent_happy() {
    let skills = SuccessAgent.skills(AgentSkillsRequest).unwrap().skills;
    assert_eq!(skills.len(), 1);
}

/// @covers: Agent::skills
#[test]
fn test_skills_agent_error() {
    let skills = FailingAgent.skills(AgentSkillsRequest).unwrap().skills;
    assert_eq!(skills.len(), 0);
}

/// @covers: Agent::skills
#[test]
fn test_skills_agent_edge() {
    let skills = EmptyAgent.skills(AgentSkillsRequest).unwrap().skills;
    assert!(skills.is_empty());
}

// ============================================================================
// Skill::name Tests
// ============================================================================

/// @covers: Skill::name
#[test]
fn test_name_skill_happy() {
    assert_eq!(
        TestSkill.name(SkillNameRequest).unwrap().name,
        "test_skill_name"
    );
}

/// @covers: Skill::name
#[test]
fn test_name_skill_error() {
    assert_eq!(MinimalSkill.name(SkillNameRequest).unwrap().name, "minimal");
}

/// @covers: Skill::name
#[test]
fn test_name_skill_edge() {
    let name = TestSkill.name(SkillNameRequest).unwrap().name;
    assert!(!name.is_empty());
}

// ============================================================================
// Skill::description Tests
// ============================================================================

/// @covers: Skill::description
#[test]
fn test_description_skill_happy() {
    assert_eq!(
        TestSkill
            .description(SkillDescriptionRequest)
            .unwrap()
            .description,
        "Test skill for integration tests"
    );
}

/// @covers: Skill::description
#[test]
fn test_description_skill_error() {
    assert_eq!(
        MinimalSkill
            .description(SkillDescriptionRequest)
            .unwrap()
            .description,
        ""
    );
}

/// @covers: Skill::description
#[test]
fn test_description_skill_edge() {
    let desc = TestSkill
        .description(SkillDescriptionRequest)
        .unwrap()
        .description;
    assert!(!desc.is_empty());
}

// ============================================================================
// Skill::parameters Tests
// ============================================================================

/// @covers: Skill::parameters
#[test]
fn test_parameters_skill_happy() {
    let params = TestSkill
        .parameters(SkillParametersRequest)
        .unwrap()
        .parameters;
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name, "input");
    assert!(params[0].required);
    assert_eq!(params[1].name, "optional");
    assert!(!params[1].required);
}

/// @covers: Skill::parameters
#[test]
fn test_parameters_skill_error() {
    let params = MinimalSkill
        .parameters(SkillParametersRequest)
        .unwrap()
        .parameters;
    assert_eq!(params.len(), 0);
}

/// @covers: Skill::parameters
#[test]
fn test_parameters_skill_edge() {
    let params = TestSkill
        .parameters(SkillParametersRequest)
        .unwrap()
        .parameters;
    assert!(params.iter().all(|p| !p.name.is_empty()));
    assert!(params.iter().all(|p| !p.param_type.is_empty()));
}

// ============================================================================
// Skill::metadata Tests
// ============================================================================

/// @covers: Skill::metadata
#[test]
fn test_metadata_skill_happy() {
    let meta = TestSkill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata;
    assert_eq!(meta.name, "test_skill_name");
    assert_eq!(meta.description, "Test skill for integration tests");
    assert!(meta.input_schema.is_some());
    assert!(meta.output_schema.is_some());
    assert!(meta.async_execution);
    assert!(!meta.long_running);
}

/// @covers: Skill::metadata
#[test]
fn test_metadata_skill_error() {
    let meta = MinimalSkill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata;
    assert_eq!(meta.name, "minimal");
    assert_eq!(meta.description, "");
    assert!(meta.input_schema.is_none());
}

/// @covers: Skill::metadata
#[test]
fn test_metadata_skill_edge() {
    let meta = TestSkill
        .metadata(SkillMetadataLookupRequest)
        .unwrap()
        .metadata;
    assert_eq!(meta.name, TestSkill.name(SkillNameRequest).unwrap().name);
    assert_eq!(
        meta.description,
        TestSkill
            .description(SkillDescriptionRequest)
            .unwrap()
            .description
    );
}

// ============================================================================
// AgentManager::load_agent Tests
// ============================================================================

/// @covers: AgentManager::load_agent
#[test]
fn test_load_agent_manager_happy() {
    let manager = TestAgentManager::new();
    let result = futures::executor::block_on(manager.load_agent(AgentLoadRequest {
        spec: "success.yaml",
    }));
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().agent.id(AgentIdRequest).unwrap().id,
        "success_agent"
    );
}

/// @covers: AgentManager::load_agent
#[test]
fn test_load_agent_manager_error() {
    let manager = TestAgentManager::new().with_failure(true);
    let result =
        futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "any.yaml" }));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

/// @covers: AgentManager::load_agent
#[test]
fn test_load_agent_manager_edge() {
    let manager = TestAgentManager::new();
    let result = futures::executor::block_on(manager.load_agent(AgentLoadRequest { spec: "" }));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

// ============================================================================
// AgentManager::agent Tests
// ============================================================================

/// @covers: AgentManager::agent
#[test]
fn test_agent_manager_happy() {
    let manager = TestAgentManager::new();
    let result = manager.agent(AgentLookupRequest {
        id: "success_agent",
    });
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().agent.id(AgentIdRequest).unwrap().id,
        "success_agent"
    );
}

/// @covers: AgentManager::agent
#[test]
fn test_agent_manager_error() {
    let manager = TestAgentManager::new();
    let result = manager.agent(AgentLookupRequest { id: "nonexistent" });
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: AgentManager::agent
#[test]
fn test_agent_manager_edge() {
    let manager = TestAgentManager::new();
    let result = manager.agent(AgentLookupRequest { id: "" });
    assert!(result.is_err());
}

// ============================================================================
// AgentManager::list_agent_ids Tests
// ============================================================================

/// @covers: AgentManager::list_agent_ids
#[test]
fn test_list_agent_ids_manager_happy() {
    let manager = TestAgentManager::new();
    let result = manager.list_agent_ids(ListAgentIdsRequest);
    assert!(result.is_ok());
    let ids = result.unwrap().ids;
    assert!(ids.len() >= 2);
    assert!(ids.contains(&"success_agent".to_string()));
    assert!(ids.contains(&"failing_agent".to_string()));
}

/// @covers: AgentManager::list_agent_ids
#[test]
fn test_list_agent_ids_manager_error() {
    // Note: In this implementation, list_agent_ids always succeeds.
    // Testing that it's callable and returns the same ids on repeated calls.
    let manager = TestAgentManager::new();
    let first = manager.list_agent_ids(ListAgentIdsRequest).unwrap().ids;
    let second = manager.list_agent_ids(ListAgentIdsRequest).unwrap().ids;
    assert_eq!(first, second);
}

/// @covers: AgentManager::list_agent_ids
#[test]
fn test_list_agent_ids_manager_edge() {
    // Create a manager with no agents
    let manager = TestAgentManager {
        agents: std::collections::HashMap::new(),
        fail_load: false,
    };
    let result = manager.list_agent_ids(ListAgentIdsRequest);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().ids.len(), 0);
}

// ============================================================================
// AgentRegistry::metadata Tests
// ============================================================================

/// @covers: AgentRegistry::metadata
#[test]
fn test_metadata_registry_happy() {
    let registry = TestAgentRegistry::new();
    let result = registry.metadata(AgentMetadataLookupRequest { id: "test_agent" });
    assert!(result.is_ok());
    let meta = result.unwrap().metadata;
    assert_eq!(meta.id, "test_agent");
    assert_eq!(meta.name, "Success Agent");
    assert_eq!(meta.version, "1.0.0");
    assert_eq!(meta.patterns.len(), 1);
}

/// @covers: AgentRegistry::metadata
#[test]
fn test_metadata_registry_error() {
    let registry = TestAgentRegistry::new();
    let result = registry.metadata(AgentMetadataLookupRequest { id: "nonexistent" });
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: AgentRegistry::metadata
#[test]
fn test_metadata_registry_edge() {
    let registry = TestAgentRegistry::empty();
    let result = registry.metadata(AgentMetadataLookupRequest { id: "any" });
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

// ============================================================================
// Registry Inherited Methods Tests (via AgentRegistry)
// ============================================================================

/// @covers: Registry::get (inherited by AgentRegistry)
#[test]
fn test_registry_get_happy() {
    let registry = TestAgentRegistry::new();
    let agent = registry.get("test_agent");
    assert!(agent.is_some());
    assert_eq!(
        agent.unwrap().id(AgentIdRequest).unwrap().id,
        "success_agent"
    );
}

/// @covers: Registry::get (inherited by AgentRegistry)
#[test]
fn test_registry_get_error() {
    let registry = TestAgentRegistry::new();
    let agent = registry.get("missing");
    assert!(agent.is_none());
}

/// @covers: Registry::get (inherited by AgentRegistry)
#[test]
fn test_registry_get_edge() {
    let registry = TestAgentRegistry::empty();
    let agent = registry.get("any");
    assert!(agent.is_none());
}

/// @covers: Registry::list_ids (inherited by AgentRegistry)
#[test]
fn test_registry_list_ids_happy() {
    let registry = TestAgentRegistry::new();
    let ids = registry.list_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"test_agent".to_string()));
}

/// @covers: Registry::list_ids (inherited by AgentRegistry)
#[test]
fn test_registry_list_ids_error() {
    let registry = TestAgentRegistry::empty();
    let ids = registry.list_ids();
    assert_eq!(ids.len(), 0);
}

/// @covers: Registry::list_ids (inherited by AgentRegistry)
#[test]
fn test_registry_list_ids_edge() {
    let registry = TestAgentRegistry::new();
    let ids = registry.list_ids();
    assert!(ids.iter().all(|id| !id.is_empty()));
}

/// @covers: Registry::len (inherited by AgentRegistry)
#[test]
fn test_registry_len_happy() {
    let registry = TestAgentRegistry::new();
    assert_eq!(registry.len(), 2);
}

/// @covers: Registry::len (inherited by AgentRegistry)
#[test]
fn test_registry_len_error() {
    let registry = TestAgentRegistry::empty();
    assert_eq!(registry.len(), 0);
}

/// @covers: Registry::len (inherited by AgentRegistry)
#[test]
fn test_registry_len_edge() {
    let registry = TestAgentRegistry::new();
    assert!(registry.len() > 0);
}

/// @covers: Registry::is_empty (inherited by AgentRegistry)
#[test]
fn test_registry_is_empty_happy() {
    let registry = TestAgentRegistry::empty();
    assert!(registry.is_empty());
}

/// @covers: Registry::is_empty (inherited by AgentRegistry)
#[test]
fn test_registry_is_empty_error() {
    let registry = TestAgentRegistry::new();
    assert!(!registry.is_empty());
}

/// @covers: Registry::is_empty (inherited by AgentRegistry)
#[test]
fn test_registry_is_empty_edge() {
    let registry = TestAgentRegistry::new();
    assert_eq!(registry.is_empty(), registry.len() == 0);
}

/// @covers: Agent::skill happy path
#[test]
fn test_skill_agent_happy_returns_result() {
    let agent = SuccessAgent;
    // SuccessAgent's only registered skill is named "test_skill_name" (see TestSkill::name).
    let result = agent.skill(SkillLookupRequest {
        name: "test_skill_name",
    });
    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().skill.name(SkillNameRequest).unwrap().name,
        "test_skill_name"
    );
}

/// @covers: Agent::skill error path
#[test]
fn test_skill_agent_error_not_found() {
    let agent = FailingAgent;
    let result = agent.skill(SkillLookupRequest {
        name: "nonexistent",
    });
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
}

/// @covers: Agent::skill edge case empty name
#[test]
fn test_skill_agent_edge_empty_name() {
    let agent = EmptyAgent;
    let result = agent.skill(SkillLookupRequest { name: "" });
    assert!(result.is_err());
}
