use serde::{Deserialize, Serialize};
use swe_edge_configbuilder::ConfigSection;

/// Provider configuration with credential source + HTTP + model defaults.
///
/// Integrates with ADR-015 (security layer) credential infrastructure and extends
/// with per-provider model defaults and HTTP configuration.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ProviderConfig {
    // Model identity and capability flags
    /// Model name/ID
    pub model: String,

    /// Temperature (0.0..=2.0)
    pub temperature: f32,

    /// API base URL (for custom/local endpoints)
    pub api_base: Option<String>,

    /// Maximum context window
    pub max_context_tokens: u32,

    /// Supports vision/image input
    pub supports_vision: bool,

    /// Supports function calling
    pub supports_functions: bool,

    /// Supports streaming
    pub supports_streaming: bool,

    // Credential source configuration (from ADR-015 Tier 2a)
    /// Credential source: env var, file path, or override env var. From swe-edge-security.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_source: Option<serde_json::Value>, // CredentialSourceConfig serialized

    // Per-provider model defaults
    /// Maximum tokens for completion responses (override global default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// System prompt/instructions (provider-specific)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,

    /// top_p (nucleus sampling, 0.0..=1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// top_k (top-K sampling)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,

    // HTTP transport configuration
    /// HTTP request timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_timeout_secs: Option<u64>,

    /// HTTP connection timeout in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_connect_timeout_secs: Option<u64>,
}

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
    pub fn with_credential_source(mut self, source: serde_json::Value) -> Self {
        self.credential_source = Some(source);
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
