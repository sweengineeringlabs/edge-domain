//! [`Token`] inherent and trait impls.

use crate::Token;

impl Token {
    /// Construct a [`Token`] from any string-like value.
    pub(crate) fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Return the token value as a string slice.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the token and return the inner string.
    pub(crate) fn into_string(self) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_token() {
        let token = Token::new("my-token");
        assert_eq!(token.as_str(), "my-token");
    }

    #[test]
    fn test_as_str_returns_slice() {
        let token = Token::new("test-token");
        assert_eq!(token.as_str(), "test-token");
    }

    #[test]
    fn test_into_string_consumes_token() {
        let token = Token::new("value");
        let value = token.into_string();
        assert_eq!(value, "value");
    }

    #[test]
    fn test_debug_redacts_value() {
        let token = Token::new("secret");
        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("[REDACTED]"));
        assert!(!debug_str.contains("secret"));
    }
}
