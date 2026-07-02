//! Constructors and `ConfigSection` impl for [`ProviderConfig`].

use swe_edge_configbuilder::ConfigSection;

use crate::api::{JsonValue, ProviderConfig};

impl ProviderConfig {
    /// Create a new provider config with required fields.
    ///
    /// All credential, model defaults, and HTTP config are optional and default to None.
    pub fn new(model: String, temperature: f32, max_context_tokens: u32) -> Self {
        Self {
            model,
            temperature,
            api_base: None,
            max_context_tokens,
            supports_vision: false,
            supports_functions: false,
            supports_streaming: false,
            credential_source: None,
            max_tokens: None,
            system_prompt: None,
            top_p: None,
            top_k: None,
            http_timeout_secs: None,
            http_connect_timeout_secs: None,
        }
    }

    /// Set credential source configuration.
    pub fn with_credential_source(mut self, source: impl Into<JsonValue>) -> Self {
        self.credential_source = Some(source.into());
        self
    }

    /// Set per-provider model defaults.
    pub fn with_model_defaults(
        mut self,
        max_tokens: Option<u32>,
        system_prompt: Option<String>,
        top_p: Option<f32>,
        top_k: Option<u32>,
    ) -> Self {
        self.max_tokens = max_tokens;
        self.system_prompt = system_prompt;
        self.top_p = top_p;
        self.top_k = top_k;
        self
    }

    /// Set HTTP transport timeouts.
    pub fn with_http_config(
        mut self,
        timeout_secs: Option<u64>,
        connect_timeout_secs: Option<u64>,
    ) -> Self {
        self.http_timeout_secs = timeout_secs;
        self.http_connect_timeout_secs = connect_timeout_secs;
        self
    }
}

impl ConfigSection for ProviderConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — TOML section key for this type
        "llm.provider"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_required_fields() {
        let config = ProviderConfig::new("claude".to_string(), 0.7, 8192);
        assert_eq!(config.model, "claude");
        assert_eq!(config.max_context_tokens, 8192);
    }

    /// @covers: with_credential_source
    #[test]
    fn test_with_credential_source_sets_field() {
        let config = ProviderConfig::new("claude".to_string(), 0.7, 8192)
            .with_credential_source(serde_json::json!({"env": "API_KEY"}));
        assert_eq!(
            config.credential_source,
            Some(JsonValue::from(serde_json::json!({"env": "API_KEY"})))
        );
    }

    /// @covers: with_model_defaults
    #[test]
    fn test_with_model_defaults_sets_all_fields() {
        let config = ProviderConfig::new("claude".to_string(), 0.7, 8192).with_model_defaults(
            Some(1024),
            Some("be helpful".to_string()),
            Some(0.9),
            Some(40),
        );
        assert_eq!(config.max_tokens, Some(1024));
        assert_eq!(config.top_k, Some(40));
    }

    /// @covers: with_http_config
    #[test]
    fn test_with_http_config_sets_timeouts() {
        let config = ProviderConfig::new("claude".to_string(), 0.7, 8192)
            .with_http_config(Some(30), Some(5));
        assert_eq!(config.http_timeout_secs, Some(30));
        assert_eq!(config.http_connect_timeout_secs, Some(5));
    }

    /// @covers: section_name
    #[test]
    fn test_section_name_is_llm_provider() {
        assert_eq!(ProviderConfig::section_name(), "llm.provider");
    }
}
