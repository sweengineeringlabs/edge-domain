#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Spec trait is exported from the crate root.

use edge_domain::Spec;

struct Even;
impl Spec<i32> for Even {
    fn matches(&self, n: &i32) -> bool {
        n % 2 == 0
    }
}

#[test]
fn test_spec_svc_facade_matches_even_returns_true() {
    assert!(Even.matches(&4));
}

#[test]
fn test_spec_svc_facade_matches_odd_returns_false() {
    assert!(!Even.matches(&7));
}
