//! Constructors and accessors for [`CacheControl`].

use crate::api::CacheControl;

impl CacheControl {
    /// Creates an ephemeral cache control hint.
    pub fn ephemeral() -> Self {
        Self {
            cache_type: "ephemeral".into(),
        }
    }

    /// Creates a cache control hint with a custom cache type.
    pub fn custom(cache_type: impl Into<String>) -> Self {
        Self {
            cache_type: cache_type.into(),
        }
    }

    /// Returns true if this cache control is ephemeral.
    pub fn is_ephemeral(&self) -> bool {
        self.cache_type == "ephemeral"
    }
}

impl Default for CacheControl {
    fn default() -> Self {
        Self::ephemeral()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: ephemeral
    #[test]
    fn test_ephemeral_sets_ephemeral_type() {
        assert!(CacheControl::ephemeral().is_ephemeral());
    }

    /// @covers: default
    #[test]
    fn test_default_is_ephemeral() {
        assert!(CacheControl::default().is_ephemeral());
    }

    /// @covers: custom
    #[test]
    fn test_custom_sets_given_type() {
        assert_eq!(CacheControl::custom("persistent").cache_type, "persistent");
    }

    /// @covers: is_ephemeral
    #[test]
    fn test_is_ephemeral_false_for_custom() {
        assert!(!CacheControl::custom("persistent").is_ephemeral());
    }
}
