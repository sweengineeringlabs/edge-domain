//! Constructors for [`CompletionInput`].

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
}
