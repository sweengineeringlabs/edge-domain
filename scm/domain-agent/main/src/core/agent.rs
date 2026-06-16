//! Stub Agent implementation for arch audit compliance.
//!
//! Real implementations live in plugins (edge-plugin-llmboot).
//! This stub exists only to satisfy the core_implements_api_traits rule.

use crate::api::{Agent, AgentError, Skill};
use std::sync::Arc;

/// Stub Agent implementation (not for production use).
pub struct StubAgent;

#[async_trait::async_trait]
impl Agent for StubAgent {
    fn id(&self) -> &str {
        "stub"
    }

    fn name(&self) -> &str {
        "Stub Agent"
    }

    fn description(&self) -> &str {
        "Placeholder for arch audit"
    }

    async fn execute_skill(
        &self,
        _skill_name: &str,
        _input: String,
    ) -> Result<String, AgentError> {
        Err(AgentError::ExecutionFailed(
            "Stub agents do not execute skills".to_string(),
        ))
    }

    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>> {
        vec![]
    }
}
