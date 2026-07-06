//! Agent trait — autonomous entity that executes skills.

use crate::api::types::{
    AgentDescriptionRequest, AgentDescriptionResponse, AgentIdRequest, AgentIdResponse,
    AgentNameRequest, AgentNameResponse, AgentProviderRequest, AgentProviderResponse,
    AgentSkillsRequest, AgentSkillsResponse, MessageBuilderRequest, MessageBuilderResponse,
    MessageSendRequest, MessageSendResponse, SkillExecutionRequest, SkillExecutionResponse,
    SkillLookupRequest, SkillLookupResponse, SupportedRoleRequest, SupportedRoleResponse,
    ToolChoicePreferenceRequest, ToolChoicePreferenceResponse,
};
use crate::api::AgentError;

/// An Agent is an autonomous entity that pursues goals through skill execution.
///
/// Agents maintain state and can execute named skills with input to produce output.
#[async_trait::async_trait]
pub trait Agent: Send + Sync {
    /// Unique agent identifier (e.g., "chief_engineer", "code_reviewer").
    fn id(&self, req: AgentIdRequest) -> Result<AgentIdResponse, AgentError>;

    /// Human-readable agent name.
    fn name(&self, req: AgentNameRequest) -> Result<AgentNameResponse, AgentError>;

    /// Agent description and purpose.
    fn description(
        &self,
        req: AgentDescriptionRequest,
    ) -> Result<AgentDescriptionResponse, AgentError>;

    /// Execute a named skill with input.
    ///
    /// Returns serialized output from the skill execution.
    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<SkillExecutionResponse, AgentError>;

    /// List all available skills.
    fn skills(&self, req: AgentSkillsRequest) -> Result<AgentSkillsResponse, AgentError>;

    /// Get a specific skill by name.
    fn skill(&self, req: SkillLookupRequest<'_>) -> Result<SkillLookupResponse, AgentError> {
        let name = req.name;
        self.skills(AgentSkillsRequest)?
            .skills
            .into_iter()
            .find(|s| {
                s.name(crate::api::types::SkillNameRequest)
                    .map(|r| r.name == name)
                    .unwrap_or(false)
            })
            .map(|skill| SkillLookupResponse { skill })
            .ok_or_else(|| AgentError::SkillNotFound(name.to_string()))
    }

    /// Append a conversation message to the agent and report the running turn count.
    ///
    /// The default implementation is stateless: it acknowledges the message
    /// without retaining it, returning `1` for the single accepted message.
    /// Stateful agents override this to accumulate conversation history.
    fn send(&self, req: MessageSendRequest) -> Result<MessageSendResponse, AgentError> {
        let _ = req.message;
        Ok(MessageSendResponse { delivered: 1 })
    }

    /// The conversation role this agent speaks as.
    ///
    /// Defaults to [`Role::Assistant`](crate::api::types::Role::Assistant); specialised agents may override.
    fn supported_role(
        &self,
        _req: SupportedRoleRequest,
    ) -> Result<SupportedRoleResponse, AgentError> {
        Ok(SupportedRoleResponse {
            role: crate::api::types::Role::Assistant,
        })
    }

    /// The tool-invocation policy this agent applies to its skills.
    ///
    /// Defaults to [`ToolChoice::Auto`](crate::api::types::ToolChoice::Auto), letting the agent decide whether to
    /// invoke a tool.
    fn tool_choice(
        &self,
        _req: ToolChoicePreferenceRequest,
    ) -> Result<ToolChoicePreferenceResponse, AgentError> {
        Ok(ToolChoicePreferenceResponse {
            choice: crate::api::types::ToolChoice::Auto,
        })
    }

    /// The LLM provider this agent delegates completions to.
    ///
    /// Every agent must be backed by a concrete provider that carries model
    /// identity, health state, and the [`edge_llm_complete::Completer`] port.
    /// Callers use the returned handle for health checks, token-budget tracking,
    /// and capability routing before invoking `execute_skill`.
    fn provider(&self, req: AgentProviderRequest) -> Result<AgentProviderResponse, AgentError>;

    /// Start a fluent [`MessageBuilder`](crate::api::types::MessageBuilder) for composing a message to this agent.
    fn message_builder(
        &self,
        _req: MessageBuilderRequest,
    ) -> Result<MessageBuilderResponse, AgentError> {
        Ok(MessageBuilderResponse {
            builder: Box::new(crate::api::types::MessageBuilder::new()),
        })
    }
}
