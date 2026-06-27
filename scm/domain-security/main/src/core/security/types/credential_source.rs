//! [`CredentialSource`] inherent and trait impls.

use crate::CredentialSource;

impl CredentialSource {
    /// Construct a [`CredentialSource`] from any string-like value.
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Return the credential name as a string slice.
    pub(crate) fn name(&self) -> &str {
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
