//! Agent trait — autonomous entity that executes skills.

use std::sync::Arc;

use edge_domain_handler::HandlerContext;
use edge_llm_provider::Provider;

use super::skill::Skill;
use crate::api::types::MessageBuilder;
use crate::api::types::{Message, Role, ToolChoice};
use crate::api::AgentError;

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
    /// * `input` - Serialized input (typically JSON) to the skill
    /// * `ctx` - Handler context with security principal and observer seam
    ///
    /// # Returns
    /// Serialized output from the skill execution
    async fn execute_skill(
        &self,
        skill_name: &str,
        input: String,
        ctx: HandlerContext<'_>,
    ) -> Result<String, AgentError>;

    /// List all available skills.
    fn skills(&self) -> Vec<Arc<dyn Skill<Request = String, Response = String>>>;

    /// Get a specific skill by name.
    fn skill(
        &self,
        name: &str,
    ) -> Result<Arc<dyn Skill<Request = String, Response = String>>, AgentError> {
        self.skills()
            .into_iter()
            .find(|s| s.name() == name)
            .ok_or_else(|| AgentError::SkillNotFound(name.to_string()))
    }

    /// Append a conversation message to the agent and report the running turn count.
    ///
    /// The default implementation is stateless: it acknowledges the message
    /// without retaining it, returning `1` for the single accepted message.
    /// Stateful agents override this to accumulate conversation history.
    fn send(&self, message: Message) -> usize {
        let _ = message;
        1
    }

    /// The conversation role this agent speaks as.
    ///
    /// Defaults to [`Role::Assistant`]; specialised agents may override.
    fn supported_role(&self) -> Role {
        Role::Assistant
    }

    /// The tool-invocation policy this agent applies to its skills.
    ///
    /// Defaults to [`ToolChoice::Auto`], letting the agent decide whether to
    /// invoke a tool.
    fn tool_choice(&self) -> ToolChoice {
        ToolChoice::Auto
    }

    /// The LLM provider this agent delegates completions to.
    ///
    /// Every agent must be backed by a concrete provider that carries model
    /// identity, health state, and the [`edge_llm_complete::Completer`] port.
    /// Callers use the returned handle for health checks, token-budget tracking,
    /// and capability routing before invoking `execute_skill`.
    fn provider(&self) -> Arc<dyn Provider>;

    /// Start a fluent [`MessageBuilder`] for composing a message to this agent.
    fn message_builder(&self) -> MessageBuilder {
        MessageBuilder::new()
    }
}
