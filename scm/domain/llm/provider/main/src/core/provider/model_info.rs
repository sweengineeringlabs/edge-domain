//! Constructors and `ConfigSection` impl for [`ModelInfo`].

use swe_edge_configbuilder::ConfigSection;

use crate::api::{ModelFamily, ModelInfo};

impl ModelInfo {
    /// Create a new model info
    pub fn new(id: String, name: String, family: ModelFamily, context_window: u32) -> Self {
        Self {
            id,
            name,
            family,
            context_window,
            supports_vision: false,
            supports_functions: false,
            supports_streaming: false,
            training_cutoff: None,
        }
    }
}

impl ConfigSection for ModelInfo {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — TOML section key for this type
        "llm.model"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_core_fields() {
        let info = ModelInfo::new(
            "gpt-4".to_string(),
            "GPT-4".to_string(),
            ModelFamily::OpenAI,
            128_000,
        );
        assert_eq!(info.id, "gpt-4");
        assert!(!info.supports_vision);
    }

    /// @covers: section_name
    #[test]
    fn test_section_name_is_llm_model() {
        assert_eq!(ModelInfo::section_name(), "llm.model");
    }
}
