//! Integration tests for [`AnonymousPrincipal`] type.

use edge_domain_security::{AnonymousPrincipal, Principal};

#[test]
fn test_anonymous_principal_id_happy() {
    let principal = AnonymousPrincipal;
    assert_eq!(
        principal.id(),
        "anonymous",
        "AnonymousPrincipal must have 'anonymous' id"
    );
}

#[test]
fn test_anonymous_principal_id_error() {
    let principal = AnonymousPrincipal;
    let id = principal.id();
    assert!(!id.is_empty(), "Principal id must not be empty");
}

#[test]
fn test_anonymous_principal_id_edge() {
    let p1 = AnonymousPrincipal;
    let p2 = AnonymousPrincipal;
    assert_eq!(
        p1.id(),
        p2.id(),
        "AnonymousPrincipal instances must have same id"
    );
}

#[test]
fn test_anonymous_principal_kind_happy() {
    let principal = AnonymousPrincipal;
    assert_eq!(
        principal.kind(),
        "anonymous",
        "AnonymousPrincipal must have 'anonymous' kind"
    );
}

#[test]
fn test_anonymous_principal_kind_error() {
    let principal = AnonymousPrincipal;
    let kind = principal.kind();
    assert!(!kind.is_empty(), "Principal kind must not be empty");
}

#[test]
fn test_anonymous_principal_kind_edge() {
    let p1 = AnonymousPrincipal;
    let p2 = AnonymousPrincipal;
    assert_eq!(
        p1.kind(),
        p2.kind(),
        "AnonymousPrincipal instances must have same kind"
    );
}
