//! `ToolCallDeltaBuilder` — fluent builder for [`ToolCallDelta`].

use crate::api::provider::types::ToolCallDelta;

/// Fluent builder for [`ToolCallDelta`].
#[derive(Clone, Debug, Default)]
pub struct ToolCallDeltaBuilder {
    index: usize,
    id: Option<String>,
    name: Option<String>,
    arguments: Option<String>,
}

impl ToolCallDeltaBuilder {
    /// Start a new builder at the given stream index.
    pub fn new(index: usize) -> Self {
        Self {
            index,
            ..Self::default()
        }
    }

    /// Set the tool-call id.
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }

    /// Set the tool name.
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }

    /// Set the (partial) arguments fragment.
    pub fn arguments(mut self, value: String) -> Self {
        self.arguments = Some(value);
        self
    }

    /// Build the [`ToolCallDelta`].
    pub fn build(self) -> ToolCallDelta {
        let mut delta = ToolCallDelta::new(self.index);
        delta.id = self.id;
        delta.name = self.name;
        delta.arguments = self.arguments;
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::ToolCallDeltaBuilder;

    #[test]
    fn test_tool_call_delta_builder_applies_overrides() {
        let delta = ToolCallDeltaBuilder::new(2)
            .id("call_1".to_string())
            .name("search".to_string())
            .arguments("{\"q\":\"x\"}".to_string())
            .build();
        assert_eq!(delta.index, 2);
        assert_eq!(delta.id.as_deref(), Some("call_1"));
        assert_eq!(delta.name.as_deref(), Some("search"));
        assert!(delta.arguments.is_some());
    }

    #[test]
    fn test_tool_call_delta_builder_defaults() {
        let delta = ToolCallDeltaBuilder::new(0).build();
        assert_eq!(delta.index, 0);
        assert!(delta.id.is_none());
    }
}
