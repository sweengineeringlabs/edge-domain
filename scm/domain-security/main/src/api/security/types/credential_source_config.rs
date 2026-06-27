//! [`CredentialSourceConfig`] — specifies where to find outbound credentials.
//!
//! Plugins declare their credential sources without hardcoding paths or env var names.
//! The framework resolves these sources at runtime using [`CredentialSourceResolver`].

use serde::{Deserialize, Serialize};

/// Specifies credential source(s) with fallback strategy.
///
/// Multiple sources can be specified; resolution tries them in order:
/// 1. `file_path_env_override` — if set, read credential file from this env var
/// 2. `file_path` — if set, read credential file from this literal path
/// 3. `env_var` — if set, read credential value directly from this env var
/// 4. Error if none are available
///
/// # Example
///
/// ```text
/// [providers.anthropic.credential_source]
/// env_var = "ANTHROPIC_API_KEY"              # Fallback (lowest priority)
/// file_path = "~/.claude/.credentials.json"  # File (medium priority)
/// file_path_env_override = "CLAUDE_CREDS_PATH" # Override (highest priority)
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct CredentialSourceConfig {
    /// Environment variable containing the credential directly.
    ///
    /// Used as last resort if file paths are not available.
    /// Example: `"ANTHROPIC_API_KEY"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_var: Option<String>,

    /// File path where credential is stored (JSON with token/refresh info for OAuth).
    ///
    /// Supports `~` expansion for home directory.
    /// Example: `"~/.claude/.credentials.json"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,

    /// Environment variable that overrides the `file_path` location.
    ///
    /// If set and the env var is present, read credential file from that env var's value
    /// instead of `file_path`. Enables runtime path customization.
    /// Example: `"CLAUDE_CREDS_PATH"` → check `$CLAUDE_CREDS_PATH` first
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path_env_override: Option<String>,
}
