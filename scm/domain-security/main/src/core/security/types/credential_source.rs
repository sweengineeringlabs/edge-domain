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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_str() {
        let source = CredentialSource::new("my-credential");
        assert_eq!(source.name(), "my-credential");
    }

    #[test]
    fn test_new_from_string() {
        let source = CredentialSource::new("my-credential".to_string());
        assert_eq!(source.name(), "my-credential");
    }

    #[test]
    fn test_name_returns_slice() {
        let source = CredentialSource::new("test");
        let name: &str = source.name();
        assert_eq!(name, "test");
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
