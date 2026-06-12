#![allow(clippy::unwrap_used, clippy::expect_used, unused_imports)]
//! SAF facade smoke test — DomainExtension marker is exported from the crate root.

use edge_domain::DomainExtension;
use edge_domain::NoopDomainExtension;

fn requires_extension<E: DomainExtension>(_e: E) {}

#[test]
fn test_domain_extension_svc_facade_noop_satisfies_bound() {
    requires_extension(NoopDomainExtension);
}
