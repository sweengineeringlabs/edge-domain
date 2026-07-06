//! Constructor for [`ModelInfo`].

use crate::api::ModelInfo;

impl ModelInfo {
    /// Construct a model info with all capability flags set to false.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        provider: impl Into<String>,
        context_window: u32,
    ) -> Self {
        Self {
            id: Self::normalized_id(id.into()),
            name: name.into(),
            provider: provider.into(),
            context_window,
            supports_vision: false,
            supports_function_calling: false,
            supports_streaming: false,
        }
    }

    /// Strip leading/trailing whitespace from a model id.
    fn normalized_id(id: String) -> String {
        id.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_id_and_defaults_capabilities_false() {
        let info = ModelInfo::new("gpt-4", "GPT-4", "openai", 8192);
        assert_eq!(info.id, "gpt-4");
        assert!(!info.supports_vision);
    }

    /// @covers: normalized_id
    #[test]
    fn test_normalized_id_strips_whitespace() {
        assert_eq!(ModelInfo::normalized_id("  gpt-4  ".to_string()), "gpt-4");
    }
}
