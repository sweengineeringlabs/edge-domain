//! [`Token`] — opaque bearer / access token.

/// An opaque bearer or access token passed on outbound requests or extracted
/// from inbound ones.
///
/// Deliberately keeps the inner value private so callers cannot accidentally
/// log or serialise raw token bytes. Use [`Token::as_str`] only at the point
/// of transmission.
#[derive(Clone, PartialEq, Eq)]
pub struct Token(String);

impl Token {
    /// Construct a [`Token`] from any string-like value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the token value as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the token and return the inner string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Token([REDACTED])")
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Token {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}
