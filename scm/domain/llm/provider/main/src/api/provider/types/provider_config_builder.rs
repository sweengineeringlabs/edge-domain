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

    /// @covers: build
    #[test]
    fn test_provider_config_builder_defaults() {
        let config = ProviderConfigBuilder::new().build();
        assert!(config.model.is_empty());
        assert!(config.api_base.is_none());
    }

    /// @covers: model
    #[test]
    fn test_model() {
        let c = ProviderConfigBuilder::new().model("gpt-4".to_string()).build();
        assert_eq!(c.model, "gpt-4");
    }

    /// @covers: temperature
    #[test]
    fn test_temperature() {
        let c = ProviderConfigBuilder::new().temperature(0.5).build();
        assert_eq!(c.temperature, 0.5);
    }

    /// @covers: api_base
    #[test]
    fn test_api_base() {
        let c = ProviderConfigBuilder::new().api_base("https://example.com".to_string()).build();
        assert_eq!(c.api_base.as_deref(), Some("https://example.com"));
    }

    /// @covers: max_context_tokens
    #[test]
    fn test_max_context_tokens() {
        let c = ProviderConfigBuilder::new().max_context_tokens(16384).build();
        assert_eq!(c.max_context_tokens, 16384);
    }

    /// @covers: supports_vision
    #[test]
    fn test_supports_vision() {
        let c = ProviderConfigBuilder::new().supports_vision(true).build();
        assert!(c.supports_vision);
    }

    /// @covers: supports_functions
    #[test]
    fn test_supports_functions() {
        let c = ProviderConfigBuilder::new().supports_functions(true).build();
        assert!(c.supports_functions);
    }

    /// @covers: supports_streaming
    #[test]
    fn test_supports_streaming() {
        let c = ProviderConfigBuilder::new().supports_streaming(true).build();
        assert!(c.supports_streaming);
    }
}
