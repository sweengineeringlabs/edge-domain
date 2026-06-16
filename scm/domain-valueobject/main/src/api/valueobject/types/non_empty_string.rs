//! `NonEmptyString` — reference value object implementation.

/// A non-empty `String` that satisfies the [`ValueObject`] contract.
///
/// Equality and hashing are by value — two instances with the same content
/// compare equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Construct a `NonEmptyString`.
    ///
    /// Returns `Err` when `s` is an empty string.
    pub fn new(s: impl Into<String>) -> Result<Self, String> {
        let s = s.into();
        if s.is_empty() {
            Err("value must not be empty".to_string())
        } else {
            Ok(Self(s))
        }
    }

    /// Return the contained string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

