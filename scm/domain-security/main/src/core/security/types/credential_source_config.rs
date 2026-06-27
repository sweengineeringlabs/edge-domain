//! [`CredentialSourceConfig`] builder impls.

use crate::CredentialSourceConfig;

impl CredentialSourceConfig {
    /// Create a new credential source config.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Set the direct environment variable name.
    pub(crate) fn with_env_var(mut self, name: impl Into<String>) -> Self {
        self.env_var = Some(name.into());
        self
    }

    /// Set the credential file path.
    pub(crate) fn with_file_path(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Set the environment variable that overrides file path.
    pub(crate) fn with_file_path_env_override(mut self, name: impl Into<String>) -> Self {
        self.file_path_env_override = Some(name.into());
        self
    }

    /// Return whether any source is configured.
    pub(crate) fn is_empty(&self) -> bool {
        self.env_var.is_none() && self.file_path.is_none() && self.file_path_env_override.is_none()
    }
}
