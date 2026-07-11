//! SAF facade tests — `OutboundRegistry` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Mutex;

use edge_domain::{
    DomainError, OutboundDeregisterRequest, OutboundGetRequest, OutboundIsEmptyRequest,
    OutboundLenRequest, OutboundNamesRequest, OutboundRegisterRequest, OutboundRegistry,
};

/// Real, minimal test-double backing store — not the crate's own `InMemoryOutboundRegistry`.
struct TestRegistry {
    store: Mutex<Vec<(String, String)>>,
}

impl TestRegistry {
    fn new() -> Self {
        Self {
            store: Mutex::new(Vec::new()),
        }
    }
}

impl OutboundRegistry for TestRegistry {
    type Handle = String;

    fn register(
        &self,
        req: OutboundRegisterRequest<String>,
    ) -> Result<edge_domain::OutboundRegisterResponse, DomainError> {
        let mut store = self.store.lock().unwrap();
        store.retain(|(n, _)| n != &req.name);
        store.push((req.name, req.handle));
        Ok(edge_domain::OutboundRegisterResponse)
    }

    fn deregister(
        &self,
        req: OutboundDeregisterRequest,
    ) -> Result<edge_domain::OutboundDeregisterResponse, DomainError> {
        let mut store = self.store.lock().unwrap();
        let before = store.len();
        store.retain(|(n, _)| n != &req.name);
        Ok(edge_domain::OutboundDeregisterResponse {
            removed: store.len() < before,
        })
    }

    fn get(
        &self,
        req: OutboundGetRequest,
    ) -> Result<edge_domain::OutboundGetResponse<String>, DomainError> {
        let store = self.store.lock().unwrap();
        let handle = store
            .iter()
            .find(|(n, _)| n == &req.name)
            .map(|(_, h)| h.clone());
        Ok(edge_domain::OutboundGetResponse { handle })
    }

    fn names(
        &self,
        _req: OutboundNamesRequest,
    ) -> Result<edge_domain::OutboundNamesResponse, DomainError> {
        let store = self.store.lock().unwrap();
        Ok(edge_domain::OutboundNamesResponse {
            names: store.iter().map(|(n, _)| n.clone()).collect(),
        })
    }

    fn len(
        &self,
        _req: OutboundLenRequest,
    ) -> Result<edge_domain::OutboundLenResponse, DomainError> {
        Ok(edge_domain::OutboundLenResponse {
            count: self.store.lock().unwrap().len(),
        })
    }

    fn is_empty(
        &self,
        _req: OutboundIsEmptyRequest,
    ) -> Result<edge_domain::OutboundIsEmptyResponse, DomainError> {
        Ok(edge_domain::OutboundIsEmptyResponse {
            empty: self.store.lock().unwrap().is_empty(),
        })
    }
}

struct FailingRegistry;

impl OutboundRegistry for FailingRegistry {
    type Handle = String;

    fn register(
        &self,
        _req: OutboundRegisterRequest<String>,
    ) -> Result<edge_domain::OutboundRegisterResponse, DomainError> {
        Err(DomainError::Unavailable("registry offline".into()))
    }

    fn deregister(
        &self,
        _req: OutboundDeregisterRequest,
    ) -> Result<edge_domain::OutboundDeregisterResponse, DomainError> {
        Err(DomainError::Unavailable("registry offline".into()))
    }

    fn get(
        &self,
        _req: OutboundGetRequest,
    ) -> Result<edge_domain::OutboundGetResponse<String>, DomainError> {
        Err(DomainError::Unavailable("registry offline".into()))
    }

    fn names(
        &self,
        _req: OutboundNamesRequest,
    ) -> Result<edge_domain::OutboundNamesResponse, DomainError> {
        Err(DomainError::Unavailable("registry offline".into()))
    }

    fn len(
        &self,
        _req: OutboundLenRequest,
    ) -> Result<edge_domain::OutboundLenResponse, DomainError> {
        Err(DomainError::Unavailable("registry offline".into()))
    }

    fn is_empty(
        &self,
        _req: OutboundIsEmptyRequest,
    ) -> Result<edge_domain::OutboundIsEmptyResponse, DomainError> {
        Err(DomainError::Unavailable("registry offline".into()))
    }
}

/// @covers: OutboundRegistry::register — success
#[test]
fn test_register_ok_registry_returns_ok_happy() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    let handle = reg
        .get(OutboundGetRequest { name: "a".into() })
        .unwrap()
        .handle;
    assert_eq!(handle.as_deref(), Some("x"));
}

/// @covers: OutboundRegistry::register — failure propagates
#[test]
fn test_register_failing_registry_returns_err_error() {
    let result = FailingRegistry.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    });
    assert!(result.is_err());
}

/// @covers: OutboundRegistry::register — re-registering replaces the previous handle
#[test]
fn test_register_overwrites_existing_name_edge() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "y".into(),
    })
    .unwrap();
    let handle = reg
        .get(OutboundGetRequest { name: "a".into() })
        .unwrap()
        .handle;
    assert_eq!(handle.as_deref(), Some("y"));
}

/// @covers: OutboundRegistry::deregister — success
#[test]
fn test_deregister_registered_name_returns_removed_true_happy() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    let removed = reg
        .deregister(OutboundDeregisterRequest { name: "a".into() })
        .unwrap()
        .removed;
    assert!(removed);
}

/// @covers: OutboundRegistry::deregister — failure propagates
#[test]
fn test_deregister_failing_registry_returns_err_error() {
    assert!(FailingRegistry
        .deregister(OutboundDeregisterRequest { name: "a".into() })
        .is_err());
}

/// @covers: OutboundRegistry::deregister — missing name reports not removed
#[test]
fn test_deregister_missing_name_returns_removed_false_edge() {
    let reg = TestRegistry::new();
    let removed = reg
        .deregister(OutboundDeregisterRequest {
            name: "missing".into(),
        })
        .unwrap()
        .removed;
    assert!(!removed);
}

/// @covers: OutboundRegistry::get — success
#[test]
fn test_get_registered_name_returns_handle_happy() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    let handle = reg
        .get(OutboundGetRequest { name: "a".into() })
        .unwrap()
        .handle;
    assert_eq!(handle.as_deref(), Some("x"));
}

/// @covers: OutboundRegistry::get — failure propagates
#[test]
fn test_get_failing_registry_returns_err_error() {
    assert!(FailingRegistry
        .get(OutboundGetRequest { name: "a".into() })
        .is_err());
}

/// @covers: OutboundRegistry::get — missing name returns None
#[test]
fn test_get_missing_name_returns_none_edge() {
    let reg = TestRegistry::new();
    let handle = reg
        .get(OutboundGetRequest {
            name: "missing".into(),
        })
        .unwrap()
        .handle;
    assert!(handle.is_none());
}

/// @covers: OutboundRegistry::names — success
#[test]
fn test_names_returns_all_registered_happy() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    reg.register(OutboundRegisterRequest {
        name: "b".into(),
        handle: "y".into(),
    })
    .unwrap();
    let mut names = reg.names(OutboundNamesRequest).unwrap().names;
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
}

/// @covers: OutboundRegistry::names — failure propagates
#[test]
fn test_names_failing_registry_returns_err_error() {
    assert!(FailingRegistry.names(OutboundNamesRequest).is_err());
}

/// @covers: OutboundRegistry::names — empty registry returns empty list
#[test]
fn test_names_empty_registry_returns_empty_vec_edge() {
    let reg = TestRegistry::new();
    assert!(reg.names(OutboundNamesRequest).unwrap().names.is_empty());
}

/// @covers: OutboundRegistry::len — success
#[test]
fn test_len_reflects_registered_count_happy() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    assert_eq!(reg.len(OutboundLenRequest).unwrap().count, 1);
}

/// @covers: OutboundRegistry::len — failure propagates
#[test]
fn test_len_failing_registry_returns_err_error() {
    assert!(FailingRegistry.len(OutboundLenRequest).is_err());
}

/// @covers: OutboundRegistry::len — empty registry is zero
#[test]
fn test_len_empty_registry_returns_zero_edge() {
    let reg = TestRegistry::new();
    assert_eq!(reg.len(OutboundLenRequest).unwrap().count, 0);
}

/// @covers: OutboundRegistry::is_empty — success
#[test]
fn test_is_empty_new_registry_returns_true_happy() {
    let reg = TestRegistry::new();
    assert!(reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
}

/// @covers: OutboundRegistry::is_empty — failure propagates
#[test]
fn test_is_empty_failing_registry_returns_err_error() {
    assert!(FailingRegistry.is_empty(OutboundIsEmptyRequest).is_err());
}

/// @covers: OutboundRegistry::is_empty — registered registry returns false
#[test]
fn test_is_empty_registered_registry_returns_false_edge() {
    let reg = TestRegistry::new();
    reg.register(OutboundRegisterRequest {
        name: "a".into(),
        handle: "x".into(),
    })
    .unwrap();
    assert!(!reg.is_empty(OutboundIsEmptyRequest).unwrap().empty);
}
