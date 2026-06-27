//! [`CredentialSource`] — identifies an outbound credential by name.

/// Names the credential to fetch for an outbound call.
///
/// Infrastructure implementations look up the secret in their backing store
/// (Vault, Secrets Manager, env vars) using this identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CredentialSource {
    name: String,
}

impl CredentialSource {
    /// Construct a [`CredentialSource`] from any string-like value.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Return the credential name as a string slice.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for CredentialSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl From<&str> for CredentialSource {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for CredentialSource {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

