use serde::{Deserialize, Serialize};

/// Provider configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Model name/ID
    pub model: String,

    /// Temperature (0.0..=2.0)
    pub temperature: f32,

    /// API base URL
    pub api_base: Option<String>,

    /// Maximum context window
    pub max_context_tokens: u32,

    /// Supports vision/image input
    pub supports_vision: bool,

    /// Supports function calling
    pub supports_functions: bool,

    /// Supports streaming
    pub supports_streaming: bool,
}

impl ProviderConfig {
    /// Create a new provider config
    pub fn new(model: String, temperature: f32, max_context_tokens: u32) -> Self {
        Self {
            model,
            temperature,
            api_base: None,
            max_context_tokens,
            supports_vision: false,
            supports_functions: false,
            supports_streaming: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderConfig;

    #[test]
    fn test_new_sets_core_fields() {
        let config = ProviderConfig::new("gpt-4".to_string(), 0.7, 8192);
        assert_eq!(config.model, "gpt-4");
        assert_eq!(config.temperature, 0.7);
        assert_eq!(config.max_context_tokens, 8192);
    }

    #[test]
    fn test_new_defaults_capabilities_off() {
        let config = ProviderConfig::new("gpt-4".to_string(), 0.7, 8192);
        assert!(!config.supports_vision);
        assert!(!config.supports_functions);
    }

    #[test]
    fn test_provider_config_serde_roundtrip() {
        let config = ProviderConfig::new("gpt-4".to_string(), 0.7, 8192);
        let json = serde_json::to_string(&config).expect("serialize");
        let back: ProviderConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.model, "gpt-4");
    }
}
