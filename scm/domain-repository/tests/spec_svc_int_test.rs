//! SAF facade tests — `Spec` trait via direct implementation.
// @allow: no_mocks_in_integration — Spec is tested through concrete impls, not test doubles

use edge_domain_repository::Spec;

struct ExactMatch(u32);
impl Spec<u32> for ExactMatch {
    fn matches(&self, entity: &u32) -> bool {
        *entity == self.0
    }
}

struct NeverMatch;
impl Spec<u32> for NeverMatch {}

/// @covers: Spec::matches — exact match returns true
#[test]
fn test_matches_exact_value_returns_true_happy() {
    let spec = ExactMatch(42);
    assert!(spec.matches(&42));
}

/// @covers: Spec::matches — non-matching value returns false
#[test]
fn test_matches_non_matching_value_returns_false_error() {
    let spec = ExactMatch(42);
    assert!(!spec.matches(&99));
}

/// @covers: Spec::matches — default impl returns false for any value
#[test]
fn test_matches_default_impl_returns_false_for_zero_edge() {
    let spec = NeverMatch;
    assert!(!spec.matches(&0));
}
