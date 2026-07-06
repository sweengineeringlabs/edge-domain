use serde::{Deserialize, Serialize};

use crate::api::provider::types::JsonValue;

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
    pub credential_source: Option<JsonValue>, // CredentialSourceConfig serialized

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
