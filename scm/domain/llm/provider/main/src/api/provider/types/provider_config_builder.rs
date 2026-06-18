//! `ProviderConfigBuilder` — fluent builder for [`ProviderConfig`].

use crate::api::provider::types::ProviderConfig;

/// Fluent builder for [`ProviderConfig`].
#[derive(Clone, Debug, Default)]
pub struct ProviderConfigBuilder {
    model: String,
    temperature: f32,
    api_base: Option<String>,
    max_context_tokens: u32,
    supports_vision: bool,
    supports_functions: bool,
    supports_streaming: bool,
}

impl ProviderConfigBuilder {
    /// Start a new builder with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the model identifier.
    pub fn model(mut self, value: String) -> Self {
        self.model = value;
        self
    }

    /// Set the sampling temperature.
    pub fn temperature(mut self, value: f32) -> Self {
        self.temperature = value;
        self
    }

    /// Set an explicit API base URL.
    pub fn api_base(mut self, value: String) -> Self {
        self.api_base = Some(value);
        self
    }

    /// Set the maximum context window.
    pub fn max_context_tokens(mut self, value: u32) -> Self {
        self.max_context_tokens = value;
        self
    }

    /// Set vision support.
    pub fn supports_vision(mut self, value: bool) -> Self {
        self.supports_vision = value;
        self
    }

    /// Set function-calling support.
    pub fn supports_functions(mut self, value: bool) -> Self {
        self.supports_functions = value;
        self
    }

    /// Set streaming support.
    pub fn supports_streaming(mut self, value: bool) -> Self {
        self.supports_streaming = value;
        self
    }

    /// Build the [`ProviderConfig`].
    pub fn build(self) -> ProviderConfig {
        let mut config = ProviderConfig::new(self.model, self.temperature, self.max_context_tokens);
        config.api_base = self.api_base;
        config.supports_vision = self.supports_vision;
        config.supports_functions = self.supports_functions;
        config.supports_streaming = self.supports_streaming;
        config
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderConfigBuilder;

    #[test]
    fn test_provider_config_builder_applies_overrides() {
        let config = ProviderConfigBuilder::new()
            .model("claude".to_string())
            .temperature(0.5)
            .max_context_tokens(200_000)
            .api_base("https://api.example.com".to_string())
            .supports_vision(true)
            .build();
        assert_eq!(config.model, "claude");
        assert_eq!(config.max_context_tokens, 200_000);
        assert_eq!(config.api_base.as_deref(), Some("https://api.example.com"));
        assert!(config.supports_vision);
    }

    #[test]
    fn test_provider_config_builder_defaults() {
        let config = ProviderConfigBuilder::new().build();
        assert!(config.model.is_empty());
        assert!(config.api_base.is_none());
    }
}
