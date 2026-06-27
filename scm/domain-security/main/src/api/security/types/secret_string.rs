//! [`SecretString`] — a string whose value is never exposed via `Debug` or `Display`.

/// A heap-allocated string that redacts its contents in debug output.
///
/// Returned by [`CredentialResolver::resolve`](crate::CredentialResolver::resolve) so raw secrets never appear in
/// logs or error messages. Use [`SecretString::expose`] only at the point of
/// transmission and only within the transport layer.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretString(String);

impl SecretString {
    /// Wrap a string value as a [`SecretString`].
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Expose the secret value as a string slice.
    ///
    /// Call this only at the point of transmission. Never log or store the
    /// returned slice.
    pub fn expose(&self) -> &str {
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

