//! Constructors for [`CacheControl`].

use crate::api::CacheControl;

impl CacheControl {
    /// Construct a cache control of the given type string.
    pub fn new(cache_type: impl Into<String>) -> Self {
        Self {
            cache_type: Self::normalized(cache_type.into()),
        }
    }

    /// The `"ephemeral"` cache control type used for Anthropic prompt caching.
    pub fn ephemeral() -> Self {
        Self::new("ephemeral")
    }

    /// Strip leading/trailing whitespace from a cache type string.
    fn normalized(cache_type: String) -> String {
        cache_type.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_cache_type() {
        assert_eq!(CacheControl::new("x").cache_type, "x");
    }

    /// @covers: ephemeral
    #[test]
    fn test_ephemeral_sets_ephemeral_type() {
        assert_eq!(CacheControl::ephemeral().cache_type, "ephemeral");
    }

    /// @covers: normalized
    #[test]
    fn test_normalized_strips_whitespace() {
        assert_eq!(CacheControl::normalized("  x  ".to_string()), "x");
    }
}
