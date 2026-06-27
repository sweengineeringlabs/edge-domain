//! Integration tests for [`Principal`] trait.

use edge_domain_security::Principal;

struct TestPrincipal {
    id: String,
    kind: String,
}

impl Principal for TestPrincipal {
    fn id(&self) -> &str {
        &self.id
    }

    fn kind(&self) -> &str {
        &self.kind
    }
}

/// @covers: Principal::id
#[test]
fn test_id_user_happy() {
    let principal = TestPrincipal {
        id: "user123".to_string(),
        kind: "tenant".to_string(),
    };
    assert_eq!(principal.id(), "user123");
}

/// @covers: Principal::id
#[test]
fn test_id_empty_error() {
    let principal = TestPrincipal {
        id: "".to_string(),
        kind: "tenant".to_string(),
    };
    let id = principal.id();
    assert!(id.is_empty());
}

/// @covers: Principal::id
#[test]
fn test_id_consistent_edge() {
    let principal = TestPrincipal {
        id: "test-id".to_string(),
        kind: "service".to_string(),
    };
    let id1 = principal.id();
    let id2 = principal.id();
    assert_eq!(id1, id2);
}

/// @covers: Principal::kind
#[test]
fn test_kind_tenant_happy() {
    let principal = TestPrincipal {
        id: "user123".to_string(),
        kind: "tenant".to_string(),
    };
    assert_eq!(principal.kind(), "tenant");
}

/// @covers: Principal::kind
#[test]
fn test_kind_empty_error() {
    let principal = TestPrincipal {
        id: "user123".to_string(),
        kind: "".to_string(),
    };
    let kind = principal.kind();
    assert!(kind.is_empty());
}

/// @covers: Principal::kind
#[test]
fn test_kind_consistent_edge() {
    let principal = TestPrincipal {
        id: "test-id".to_string(),
        kind: "anonymous".to_string(),
    };
    let kind1 = principal.kind();
    let kind2 = principal.kind();
    assert_eq!(kind1, kind2);
}
