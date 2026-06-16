//! `Spec` — predicate type for filtering entities in a repository query.

/// A specification predicate used to filter entities of type `T`.
///
/// Implement this trait to express domain query criteria without exposing
/// persistence details.
pub trait Spec<T: Send + Sync>: Send + Sync {
    /// Returns `true` if `entity` satisfies this specification.
    fn matches(&self, _entity: &T) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysMatch;
    impl Spec<u32> for AlwaysMatch {
        fn matches(&self, _entity: &u32) -> bool {
            true
        }
    }

    struct NeverMatch;
    impl Spec<u32> for NeverMatch {}

    #[test]
    fn test_matches_custom_impl_returns_true_happy() {
        let spec = AlwaysMatch;
        assert!(spec.matches(&42));
    }

    #[test]
    fn test_matches_default_impl_returns_false_error() {
        let spec = NeverMatch;
        assert!(!spec.matches(&42));
    }

    #[test]
    fn test_matches_default_impl_consistent_across_values_edge() {
        let spec = NeverMatch;
        assert!(!spec.matches(&0));
        assert!(!spec.matches(&u32::MAX));
    }
}
