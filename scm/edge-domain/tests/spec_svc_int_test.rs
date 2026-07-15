#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — Spec trait is exported from the crate root.
#![cfg(feature = "repository")]

use edge_application::{RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse};

struct Even;
impl Spec for Even {
    type Entity = i32;

    fn matches(
        &self,
        req: SpecMatchesRequest<'_, i32>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse {
            matches: req.entity % 2 == 0,
        })
    }
}

#[test]
fn test_spec_svc_facade_matches_even_returns_true() {
    assert!(
        Even.matches(SpecMatchesRequest { entity: &4 })
            .unwrap()
            .matches
    );
}

#[test]
fn test_spec_svc_facade_matches_odd_returns_false() {
    assert!(
        !Even
            .matches(SpecMatchesRequest { entity: &7 })
            .unwrap()
            .matches
    );
}
