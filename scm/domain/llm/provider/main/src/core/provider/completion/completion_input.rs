//! Constructors and conversions for [`CompletionInput`].

use edge_llm_complete::{CompletionRequest, Message, MessageContent, Role};

use crate::api::{CompletionInput, CompletionMessage, ExecutionConfig, ToolDefinition};

impl CompletionInput {
    /// Construct a simple single-turn input with no tools and no system prompt.
    pub fn simple(prompt: impl Into<String>, config: ExecutionConfig) -> Self {
        Self {
            messages: vec![CompletionMessage::user(prompt)],
            tools: Self::no_tools(),
            system: None,
            config,
        }
    }

    /// Empty tool list shared by constructors that don't accept tools.
    fn no_tools() -> Vec<ToolDefinition> {
        vec![]
    }

    /// Construct a multi-turn input.
    pub fn new(
        messages: Vec<CompletionMessage>,
        tools: Vec<ToolDefinition>,
        system: Option<String>,
        config: ExecutionConfig,
    ) -> Self {
        Self {
            messages,
            tools,
            system,
            config,
        }
    }

    /// Convert into an [`edge_llm_complete::CompletionRequest`] bound for a
    /// [`Completer`](edge_llm_complete::Completer), given the target model id and
    /// sampling temperature (neither of which this type carries itself).
    ///
    /// The system prompt, if present, becomes a leading [`Role::System`] message.
    pub(crate) fn into_completion_request(
        self,
        model: impl Into<String>,
        temperature: f32,
    ) -> CompletionRequest {
        let mut messages: Vec<Message> = Vec::with_capacity(self.messages.len() + 1);
        if let Some(system) = self.system {
            messages.push(Self::system_message(system));
        }
        messages.extend(
            self.messages
                .into_iter()
                .map(CompletionMessage::into_message),
        );

        CompletionRequest {
            model: model.into(),
            messages,
            temperature: Some(temperature),
            max_tokens: Some(self.config.max_tokens_per_call),
            top_p: None,
            stop: None,
            tools: (!self.tools.is_empty()).then(|| {
                self.tools
                    .into_iter()
                    .map(ToolDefinition::into_complete_tool)
                    .collect()
            }),
            tool_choice: None,
        }
    }

    /// Build the leading system-role message from a system prompt string.
    fn system_message(content: String) -> Message {
        Message {
            role: Role::System,
            content: MessageContent::Text(content),
            ..Message::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ExecutionMode;

    fn config() -> ExecutionConfig {
        ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async)
    }

    /// @covers: simple
    #[test]
    fn test_simple_creates_single_user_message() {
        let input = CompletionInput::simple("hi", config());
        assert_eq!(input.messages.len(), 1);
        assert!(input.tools.is_empty());
    }

    /// @covers: new
    #[test]
    fn test_new_sets_all_fields() {
        let input = CompletionInput::new(vec![], vec![], Some("sys".to_string()), config());
        assert_eq!(input.system, Some("sys".to_string()));
    }

    /// @covers: no_tools
    #[test]
    fn test_no_tools_is_empty() {
        assert!(CompletionInput::no_tools().is_empty());
    }

    /// @covers: into_completion_request
    #[test]
    fn test_into_completion_request_sets_model_and_sampling_params() {
        let request =
            CompletionInput::simple("hi", config()).into_completion_request("claude", 0.7);
        assert_eq!(request.model, "claude");
        assert_eq!(request.temperature, Some(0.7));
        assert_eq!(request.max_tokens, Some(4096));
    }

    /// @covers: into_completion_request
    #[test]
    fn test_into_completion_request_prepends_system_message() {
        let input = CompletionInput::new(
            vec![CompletionMessage::user("hi")],
            vec![],
            Some("be terse".to_string()),
            config(),
        );
        let request = input.into_completion_request("claude", 0.0);
        assert_eq!(request.messages.len(), 2);
        assert_eq!(request.messages[0].role, Role::System);
        assert_eq!(
            request.messages[0].content,
            MessageContent::Text("be terse".to_string())
        );
    }

    /// @covers: into_completion_request
    #[test]
    fn test_into_completion_request_maps_all_message_roles() {
        let input = CompletionInput::new(
            vec![
                CompletionMessage::user("q"),
                CompletionMessage::assistant("a"),
                CompletionMessage::tool("r"),
            ],
            vec![],
            None,
            config(),
        );
        let request = input.into_completion_request("claude", 0.0);
        assert_eq!(
            request.messages.iter().map(|m| m.role).collect::<Vec<_>>(),
            vec![Role::User, Role::Assistant, Role::Tool]
        );
    }

    /// @covers: into_completion_request
    #[test]
    fn test_into_completion_request_converts_tools() {
        let tool = ToolDefinition::new("search", "search the web", crate::api::JsonValue::Null);
        let input = CompletionInput::new(vec![], vec![tool], None, config());
        let tools = input
            .into_completion_request("claude", 0.0)
            .tools
            .expect("tools should be Some when non-empty");
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "search");
    }

    /// @covers: into_completion_request
    #[test]
    fn test_into_completion_request_empty_tools_yields_none() {
        let request =
            CompletionInput::simple("hi", config()).into_completion_request("claude", 0.0);
        assert!(request.tools.is_none());
    }
}
