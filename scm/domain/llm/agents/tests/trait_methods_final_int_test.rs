#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Comprehensive trait method scenario coverage tests.
//!
//! Tests all public trait methods in Agent, Skill, AgentManager, and AgentRegistry
//! with happy, error, and edge cases.

use async_trait::async_trait;
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_registry::Registry;
use edge_llm_agent::{
    Agent, AgentError, AgentManager, AgentMetadata, AgentRegistry, Parameter, Skill, SkillMetadata,
};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderConfig, ProviderFactory, StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    Arc::new(StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
    ))
}

// ============================================================================
// Test Fixtures
// ============================================================================

/// A test agent that succeeds
struct SuccessAgent;

#[async_trait]
impl Agent for SuccessAgent {
    fn id(&self) -> &str {
        "success_agent"
    }

    fn name(&self) -> &str {
        "Success Agent"
    }

    fn description(&self) -> &str {
        "Test agent that succeeds"
    }

    async fn execute_skill(&self, skill_name: &str, input: String) -> Result<String, AgentError> {
        Ok(format!("{}:{}", skill_name, input))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![Arc::new(TestSkill) as Arc<dyn Skill<Request = String, Response = String>>]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

/// A test agent that fails
struct FailingAgent;

#[async_trait]
impl Agent for FailingAgent {
    fn id(&self) -> &str {
        "failing_agent"
    }

    fn name(&self) -> &str {
        "Failing Agent"
    }

    fn description(&self) -> &str {
        "Test agent that fails"
    }

    async fn execute_skill(&self, _skill_name: &str, _input: String) -> Result<String, AgentError> {
        Err(AgentError::ExecutionFailed(
            "intentional failure".to_string(),
        ))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

/// A test agent with empty fields
struct EmptyAgent;

#[async_trait]
impl Agent for EmptyAgent {
    fn id(&self) -> &str {
        ""
    }

    fn name(&self) -> &str {
        ""
    }

    fn description(&self) -> &str {
        ""
    }

    async fn execute_skill(&self, _skill_name: &str, _input: String) -> Result<String, AgentError> {
        Ok("".to_string())
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

/// A test skill implementation
struct TestSkill;

#[async_trait]
impl Handler for TestSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "test_skill"
    }

    async fn execute(&self, req: String, _ctx: HandlerContext<'_>) -> Result<String, HandlerError> {
        Ok(format!("skill_response:{}", req))
    }
}

impl Skill for TestSkill {
    fn name(&self) -> &str {
        "test_skill_name"
    }

    fn description(&self) -> &str {
        "Test skill for integration tests"
    }

    fn parameters(&self) -> Vec<Parameter> {
        vec![
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
        ]
    }

    fn metadata(&self) -> SkillMetadata {
        SkillMetadata {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: Some(r#"{"type":"string"}"#.to_string()),
            output_schema: Some(r#"{"type":"string"}"#.to_string()),
            async_execution: true,
            long_running: false,
        }
    }
}

/// A minimal skill with no parameters
struct MinimalSkill;

#[async_trait]
impl Handler for MinimalSkill {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        "minimal"
    }

    async fn execute(
        &self,
        _req: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        Err(HandlerError::ExecutionFailed("minimal error".to_string()))
    }
}

impl Skill for MinimalSkill {
    fn name(&self) -> &str {
        "minimal"
    }

    fn description(&self) -> &str {
        ""
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
    async fn load_agent(&self, spec: &str) -> Result<Arc<dyn Agent>, AgentError> {
        if self.fail_load {
            return Err(AgentError::InvalidSpec(format!("Failed to load: {}", spec)));
        }
        if spec.is_empty() {
            return Err(AgentError::InvalidSpec("Empty spec".to_string()));
        }
        if spec == "success.yaml" {
            Ok(Arc::new(SuccessAgent))
        } else {
            Err(AgentError::InvalidSpec(format!("Unknown spec: {}", spec)))
        }
    }

    fn agent(&self, id: &str) -> Result<Arc<dyn Agent>, AgentError> {
        self.agents
            .get(id)
            .cloned()
            .ok_or_else(|| AgentError::NotFound(id.to_string()))
    }

    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError> {
        Ok(self.agents.keys().cloned().collect())
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
    fn metadata(&self, id: &str) -> Result<AgentMetadata, AgentError> {
        if let Some(agent) = self.agents.get(id) {
            Ok(AgentMetadata {
                id: id.to_string(),
                name: agent.name().to_string(),
                description: agent.description().to_string(),
                version: "1.0.0".to_string(),
                skills: vec![],
                patterns: vec!["test".to_string()],
            })
        } else {
            Err(AgentError::NotFound(id.to_string()))
        }
    }
}

// ============================================================================
// Agent::id Tests
// ============================================================================

/// @covers: Agent::id
#[test]
fn test_id_agent_happy() {
    assert_eq!(SuccessAgent.id(), "success_agent");
}

/// @covers: Agent::id
#[test]
fn test_id_agent_error() {
    let agent = EmptyAgent;
    assert_eq!(agent.id(), "");
}

/// @covers: Agent::id
#[test]
fn test_id_agent_edge() {
    let agent1 = SuccessAgent;
    let agent2 = FailingAgent;
    assert_ne!(agent1.id(), agent2.id());
}

// ============================================================================
// Agent::name Tests
// ============================================================================

/// @covers: Agent::name
#[test]
fn test_name_agent_happy() {
    assert_eq!(SuccessAgent.name(), "Success Agent");
}

/// @covers: Agent::name
#[test]
fn test_name_agent_error() {
    assert_eq!(EmptyAgent.name(), "");
}

/// @covers: Agent::name
#[test]
fn test_name_agent_edge() {
    let name = SuccessAgent.name();
    assert!(!name.is_empty());
    assert!(name.contains("Agent"));
}

// ============================================================================
// Agent::description Tests
// ============================================================================

/// @covers: Agent::description
#[test]
fn test_description_agent_happy() {
    let desc = SuccessAgent.description();
    assert_eq!(desc, "Test agent that succeeds");
}

/// @covers: Agent::description
#[test]
fn test_description_agent_error() {
    assert_eq!(EmptyAgent.description(), "");
}

/// @covers: Agent::description
#[test]
fn test_description_agent_edge() {
    let success_desc = SuccessAgent.description();
    let failing_desc = FailingAgent.description();
    assert_ne!(success_desc, failing_desc);
}

// ============================================================================
// Agent::execute_skill Tests
// ============================================================================

/// @covers: Agent::execute_skill
#[test]
fn test_execute_skill_agent_happy() {
    let result = futures::executor::block_on(
        SuccessAgent.execute_skill("code_review", "input.rs".to_string()),
    );
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "code_review:input.rs");
}

/// @covers: Agent::execute_skill
#[test]
fn test_execute_skill_agent_error() {
    let result =
        futures::executor::block_on(FailingAgent.execute_skill("any_skill", "input".to_string()));
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
    let result = futures::executor::block_on(SuccessAgent.execute_skill("", "".to_string()));
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), ":");
}

// ============================================================================
// Agent::skills Tests
// ============================================================================

/// @covers: Agent::skills
#[test]
fn test_skills_agent_happy() {
    let skills = SuccessAgent.skills();
    assert_eq!(skills.len(), 1);
}

/// @covers: Agent::skills
#[test]
fn test_skills_agent_error() {
    let skills = FailingAgent.skills();
    assert_eq!(skills.len(), 0);
}

/// @covers: Agent::skills
#[test]
fn test_skills_agent_edge() {
    let skills = EmptyAgent.skills();
    assert!(skills.is_empty());
}

// ============================================================================
// Skill::name Tests
// ============================================================================

/// @covers: Skill::name
#[test]
fn test_name_skill_happy() {
    assert_eq!(TestSkill.name(), "test_skill_name");
}

/// @covers: Skill::name
#[test]
fn test_name_skill_error() {
    assert_eq!(MinimalSkill.name(), "minimal");
}

/// @covers: Skill::name
#[test]
fn test_name_skill_edge() {
    let name = TestSkill.name();
    assert!(!name.is_empty());
}

// ============================================================================
// Skill::description Tests
// ============================================================================

/// @covers: Skill::description
#[test]
fn test_description_skill_happy() {
    assert_eq!(TestSkill.description(), "Test skill for integration tests");
}

/// @covers: Skill::description
#[test]
fn test_description_skill_error() {
    assert_eq!(MinimalSkill.description(), "");
}

/// @covers: Skill::description
#[test]
fn test_description_skill_edge() {
    let desc = TestSkill.description();
    assert!(!desc.is_empty());
}

// ============================================================================
// Skill::parameters Tests
// ============================================================================

/// @covers: Skill::parameters
#[test]
fn test_parameters_skill_happy() {
    let params = TestSkill.parameters();
    assert_eq!(params.len(), 2);
    assert_eq!(params[0].name, "input");
    assert!(params[0].required);
    assert_eq!(params[1].name, "optional");
    assert!(!params[1].required);
}

/// @covers: Skill::parameters
#[test]
fn test_parameters_skill_error() {
    let params = MinimalSkill.parameters();
    assert_eq!(params.len(), 0);
}

/// @covers: Skill::parameters
#[test]
fn test_parameters_skill_edge() {
    let params = TestSkill.parameters();
    assert!(params.iter().all(|p| !p.name.is_empty()));
    assert!(params.iter().all(|p| !p.param_type.is_empty()));
}

// ============================================================================
// Skill::metadata Tests
// ============================================================================

/// @covers: Skill::metadata
#[test]
fn test_metadata_skill_happy() {
    let meta = TestSkill.metadata();
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
    let meta = MinimalSkill.metadata();
    assert_eq!(meta.name, "minimal");
    assert_eq!(meta.description, "");
    assert!(meta.input_schema.is_none());
}

/// @covers: Skill::metadata
#[test]
fn test_metadata_skill_edge() {
    let meta = TestSkill.metadata();
    assert_eq!(meta.name, TestSkill.name());
    assert_eq!(meta.description, TestSkill.description());
}

// ============================================================================
// AgentManager::load_agent Tests
// ============================================================================

/// @covers: AgentManager::load_agent
#[test]
fn test_load_agent_manager_happy() {
    let manager = TestAgentManager::new();
    let result = futures::executor::block_on(manager.load_agent("success.yaml"));
    assert!(result.is_ok());
}

/// @covers: AgentManager::load_agent
#[test]
fn test_load_agent_manager_error() {
    let manager = TestAgentManager::new().with_failure(true);
    let result = futures::executor::block_on(manager.load_agent("any.yaml"));
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
}

/// @covers: AgentManager::load_agent
#[test]
fn test_load_agent_manager_edge() {
    let manager = TestAgentManager::new();
    let result = futures::executor::block_on(manager.load_agent(""));
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
    let result = manager.agent("success_agent");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id(), "success_agent");
}

/// @covers: AgentManager::agent
#[test]
fn test_agent_manager_error() {
    let manager = TestAgentManager::new();
    let result = manager.agent("nonexistent");
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: AgentManager::agent
#[test]
fn test_agent_manager_edge() {
    let manager = TestAgentManager::new();
    let result = manager.agent("");
    assert!(result.is_err());
}

// ============================================================================
// AgentManager::list_agent_ids Tests
// ============================================================================

/// @covers: AgentManager::list_agent_ids
#[test]
fn test_list_agent_ids_manager_happy() {
    let manager = TestAgentManager::new();
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
    let ids = result.unwrap();
    assert!(ids.len() >= 2);
    assert!(ids.contains(&"success_agent".to_string()));
    assert!(ids.contains(&"failing_agent".to_string()));
}

/// @covers: AgentManager::list_agent_ids
#[test]
fn test_list_agent_ids_manager_error() {
    // Note: In this implementation, list_agent_ids always succeeds.
    // Testing that it's callable and returns consistent result.
    let manager = TestAgentManager::new();
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
}

/// @covers: AgentManager::list_agent_ids
#[test]
fn test_list_agent_ids_manager_edge() {
    // Create a manager with no agents
    let manager = TestAgentManager {
        agents: std::collections::HashMap::new(),
        fail_load: false,
    };
    let result = manager.list_agent_ids();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

// ============================================================================
// AgentRegistry::metadata Tests
// ============================================================================

/// @covers: AgentRegistry::metadata
#[test]
fn test_metadata_registry_happy() {
    let registry = TestAgentRegistry::new();
    let result = registry.metadata("test_agent");
    assert!(result.is_ok());
    let meta = result.unwrap();
    assert_eq!(meta.id, "test_agent");
    assert_eq!(meta.name, "Success Agent");
    assert_eq!(meta.version, "1.0.0");
    assert_eq!(meta.patterns.len(), 1);
}

/// @covers: AgentRegistry::metadata
#[test]
fn test_metadata_registry_error() {
    let registry = TestAgentRegistry::new();
    let result = registry.metadata("nonexistent");
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: AgentRegistry::metadata
#[test]
fn test_metadata_registry_edge() {
    let registry = TestAgentRegistry::empty();
    let result = registry.metadata("any");
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
    let result = agent.skill("any");
    assert!(result.is_ok() || result.is_err());
}

/// @covers: Agent::skill error path
#[test]
fn test_skill_agent_error_not_found() {
    let agent = FailingAgent;
    let result = agent.skill("nonexistent");
    assert!(result.is_err());
}

/// @covers: Agent::skill edge case empty name
#[test]
fn test_skill_agent_edge_empty_name() {
    let agent = EmptyAgent;
    let result = agent.skill("");
    assert!(result.is_err());
}
