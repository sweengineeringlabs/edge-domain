//! Integration tests for `Spec<T>` — predicate trait.

use edge_domain_repository::Spec;

struct AlwaysTrue;
impl Spec<u32> for AlwaysTrue {
    fn matches(&self, _entity: &u32) -> bool {
        true
    }
}

struct AlwaysFalse;
impl Spec<u32> for AlwaysFalse {}

struct GreaterThan(u32);
impl Spec<u32> for GreaterThan {
    fn matches(&self, entity: &u32) -> bool {
        *entity > self.0
    }
}

/// @covers: Spec::matches — custom impl returning true
#[test]
fn test_matches_always_true_spec_returns_true_happy() {
    let spec = AlwaysTrue;
    assert!(spec.matches(&0));
    assert!(spec.matches(&u32::MAX));
}

/// @covers: Spec::matches — default impl returns false
#[test]
fn test_matches_default_impl_returns_false_error() {
    let spec = AlwaysFalse;
    assert!(!spec.matches(&42));
}

/// @covers: Spec::matches — boundary value at threshold
#[test]
fn test_matches_boundary_value_at_threshold_edge() {
    let spec = GreaterThan(10);
    assert!(!spec.matches(&10));
    assert!(spec.matches(&11));
}
