//! [`CredentialSourceResolver`] — resolves CredentialSourceConfig to concrete CredentialSource.

use crate::api::{CredentialSource, CredentialSourceConfig, SecurityError};

/// Resolves a [`CredentialSourceConfig`] to a concrete [`CredentialSource`].
///
/// Handles:
/// - Environment variable overrides (`file_path_env_override` → `$OVERRIDE_VAR`)
/// - File path expansion (`~/.credentials.json` → `/home/user/.credentials.json`)
/// - Fallback to `env_var` if files not available
/// - Validation of file accessibility before declaring success
///
/// # Errors
///
/// Returns [`SecurityError::Credential`] when:
/// - No sources are configured (all `Option::None`)
/// - All configured sources are missing or inaccessible
/// - File path doesn't exist or is unreadable
pub trait CredentialSourceResolver: Send + Sync {
    /// Resolve the credential source config to a concrete source.
    ///
    /// # Resolution Order
    ///
    /// 1. If `config.file_path_env_override` is set, check that env var
    ///    - If env var points to a readable file, use it
    /// 2. If `config.file_path` is set, check that file path
    ///    - Expand `~` to home directory if needed
    ///    - If file is readable, return it as a file source
    /// 3. If `config.env_var` is set, return it as an env var source
    ///    - No validation; caller is responsible for checking if env var exists
    /// 4. If none are available, return error with diagnostic
    fn resolve(&self, config: &CredentialSourceConfig) -> Result<CredentialSource, SecurityError>;
}

