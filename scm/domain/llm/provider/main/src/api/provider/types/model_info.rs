use crate::api::provider::types::ModelFamily;
use serde::{Deserialize, Serialize};

/// LLM model metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Model ID (e.g., "gpt-4-turbo")
    pub id: String,

    /// Model name
    pub name: String,

    /// Model provider/family
    pub family: ModelFamily,

    /// Context window size
    pub context_window: u32,

    /// Supports vision/images
    pub supports_vision: bool,

    /// Supports function calling
    pub supports_functions: bool,

    /// Supports streaming
    pub supports_streaming: bool,

    /// Training data cutoff date
    pub training_cutoff: Option<String>,
}

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

#[cfg(test)]
mod tests {
    use super::ModelInfo;
    use crate::api::provider::types::ModelFamily;

    #[test]
    fn test_new_sets_core_fields() {
        let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 128_000);
        assert_eq!(info.id, "gpt-4");
        assert_eq!(info.context_window, 128_000);
        assert_eq!(info.family, ModelFamily::OpenAI);
    }

    #[test]
    fn test_new_defaults_capabilities_off() {
        let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 128_000);
        assert!(!info.supports_vision);
    }

    #[test]
    fn test_model_info_serde_roundtrip() {
        let info = ModelInfo::new("gpt-4".to_string(), "GPT-4".to_string(), ModelFamily::OpenAI, 8192);
        let json = serde_json::to_string(&info).expect("serialize");
        let back: ModelInfo = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.id, "gpt-4");
    }
}
