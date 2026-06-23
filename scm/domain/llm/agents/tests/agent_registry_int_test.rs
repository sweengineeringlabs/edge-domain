#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests — `AgentRegistry` trait.

use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_observer::StdObserveFactory;
use edge_domain_registry::Registry;
use edge_domain_security::SecurityContext;
use edge_llm_agent::{Agent, AgentError, AgentMetadata, AgentRegistry, Skill};
use edge_llm_provider::{
    EchoProviderCompleter, ModelInfo, Provider, ProviderBootstrap, ProviderConfig,
    StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        ModelInfo::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

struct DummyAgent;

#[async_trait::async_trait]
impl Agent for DummyAgent {
    fn id(&self) -> &str {
        "test"
    }

    fn name(&self) -> &str {
        "Test"
    }

    fn description(&self) -> &str {
        "Test agent"
    }

    async fn execute_skill(
        &self,
        _skill_name: &str,
        _input: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, AgentError> {
        Ok("ok".to_string())
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }

    fn provider(&self) -> Arc<dyn Provider> {
        noop_provider()
    }
}

struct TestRegistry {
    has_agent: bool,
}

impl Registry for TestRegistry {
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
        if self.has_agent && id == "found" {
            Some(Arc::new(DummyAgent))
        } else {
            None
        }
    }

    fn list_ids(&self) -> Vec<String> {
        if self.has_agent {
            vec!["agent1".to_string()]
        } else {
            vec![]
        }
    }

    fn len(&self) -> usize {
        if self.has_agent {
            1
        } else {
            0
        }
    }
}

impl AgentRegistry for TestRegistry {
    fn metadata(&self, id: &str) -> Result<AgentMetadata, AgentError> {
        if self.has_agent && id == "found" {
            Ok(AgentMetadata {
                id: id.to_string(),
                name: "Found Agent".to_string(),
                description: "Agent found in registry".to_string(),
                version: "1.0.0".to_string(),
                skills: vec![],
                patterns: vec!["react".to_string()],
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
    let result = registry.metadata("found");
    assert!(result.is_ok());
    let meta = result.unwrap();
    assert_eq!(meta.id, "found");
    assert_eq!(meta.name, "Found Agent");
}

/// @covers: AgentRegistry::metadata — not found
#[test]
fn test_trait_agent_registry_error_metadata_missing_agent_returns_not_found() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.metadata("missing");
    assert!(result.is_err());
    assert!(matches!(result, Err(AgentError::NotFound(_))));
}

/// @covers: Registry::get (inherited)
#[test]
fn test_trait_agent_registry_happy_get_existing_returns_some() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.get("found");
    assert!(result.is_some());
}

/// @covers: Registry::get (inherited) — missing
#[test]
fn test_trait_agent_registry_error_get_missing_returns_none() {
    let registry = TestRegistry { has_agent: true };
    let result = registry.get("missing");
    assert!(result.is_none());
}

/// @covers: Registry::list_ids (inherited)
#[test]
fn test_trait_agent_registry_happy_list_ids_returns_ids() {
    let registry = TestRegistry { has_agent: true };
    let ids = registry.list_ids();
    assert_eq!(ids.len(), 1);
    assert!(ids.contains(&"agent1".to_string()));
}

/// @covers: Registry::list_ids (inherited) — empty
#[test]
fn test_trait_agent_registry_edge_list_ids_empty_returns_empty_vec() {
    let registry = TestRegistry { has_agent: false };
    let ids = registry.list_ids();
    assert_eq!(ids.len(), 0);
}

/// @covers: Registry::len (inherited)
#[test]
fn test_trait_agent_registry_happy_len_with_agent_returns_one() {
    let registry = TestRegistry { has_agent: true };
    assert_eq!(registry.len(), 1);
}

/// @covers: Registry::len (inherited) — empty
#[test]
fn test_trait_agent_registry_happy_len_empty_returns_zero() {
    let registry = TestRegistry { has_agent: false };
    assert_eq!(registry.len(), 0);
}

/// @covers: Registry::is_empty (inherited)
#[test]
fn test_trait_agent_registry_happy_is_empty_when_empty_returns_true() {
    let registry = TestRegistry { has_agent: false };
    assert!(registry.is_empty());
}

/// @covers: Registry::is_empty (inherited) — not empty
#[test]
fn test_trait_agent_registry_happy_is_empty_when_has_items_returns_false() {
    let registry = TestRegistry { has_agent: true };
    assert!(!registry.is_empty());
}

/// @covers: AgentRegistry — metadata + Registry methods work together
#[test]
fn test_trait_agent_registry_happy_all_methods_together() {
    let registry = TestRegistry { has_agent: true };
    assert!(!registry.is_empty());
    assert!(registry.metadata("found").is_ok());
    assert!(registry.get("found").is_some());
    assert_eq!(registry.list_ids().len(), 1);
}
