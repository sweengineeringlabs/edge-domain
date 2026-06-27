//! [`SecretString`] inherent and trait impls.

use crate::SecretString;

impl SecretString {
    /// Wrap a string value as a [`SecretString`].
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Expose the secret value as a string slice.
    ///
    /// Call this only at the point of transmission. Never log or store the
    /// returned slice.
    pub(crate) fn expose(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SecretString([REDACTED])")
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl From<String> for SecretString {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for SecretString {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_secret() {
        let secret = SecretString::new("my-secret");
        assert_eq!(secret.expose(), "my-secret");
    }

    #[test]
    fn test_expose_returns_value() {
        let secret = SecretString::new("test-value");
        assert_eq!(secret.expose(), "test-value");
    }

    #[test]
    fn test_debug_redacts_value() {
        let secret = SecretString::new("secret");
        let debug_str = format!("{:?}", secret);
        assert!(debug_str.contains("[REDACTED]"));
        assert!(!debug_str.contains("secret"));
    }

    #[test]
    fn test_display_redacts_value() {
        let secret = SecretString::new("secret");
        let display_str = format!("{}", secret);
        assert_eq!(display_str, "[REDACTED]");
    }
}
