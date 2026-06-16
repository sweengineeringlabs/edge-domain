//! Agent trait — autonomous entity that executes skills.

use serde_json::Value;
use std::sync::Arc;

use super::skill::Skill;
use crate::AgentError;

/// An Agent is an autonomous entity that pursues goals through skill execution.
///
/// Agents maintain state and can execute named skills with input to produce output.
#[async_trait::async_trait]
pub trait Agent: Send + Sync {
    /// Unique agent identifier (e.g., "chief_engineer", "code_reviewer").
    fn id(&self) -> &str;

    /// Human-readable agent name.
    fn name(&self) -> &str;

    /// Agent description and purpose.
    fn description(&self) -> &str;

    /// Execute a named skill with input.
    ///
    /// # Arguments
    /// * `skill_name` - Name of the skill to execute (e.g., "code_review")
    /// * `input` - JSON input to the skill
    ///
    /// # Returns
    /// JSON output from the skill execution
    async fn execute_skill(&self, skill_name: &str, input: Value) -> Result<Value, AgentError>;

    /// List all available skills.
    fn skills(&self) -> Vec<Arc<dyn Skill>>;

    /// Get a specific skill by name.
    fn skill(&self, name: &str) -> Result<Arc<dyn Skill>, AgentError> {
        self.skills()
            .into_iter()
            .find(|s| s.name() == name)
            .ok_or_else(|| AgentError::SkillNotFound(name.to_string()))
    }
}
