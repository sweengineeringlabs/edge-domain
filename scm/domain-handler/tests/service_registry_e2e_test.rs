//! End-to-end contract tests for the `ServiceRegistry` trait, exercised through a
//! test-double implementation via the crate's public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_application_handler::{
    HandlerError, ListNamesRequest, Service, ServiceLookupRequest, ServiceLookupResponse,
    ServiceRegistry,
};
use edge_application_service::{NoopRequest, NoopResponse};
use futures::executor::block_on;

struct StubService;

#[async_trait::async_trait]
impl Service for StubService {
    type Request = NoopRequest;
    type Response = NoopResponse;

    async fn execute(&self, _req: NoopRequest) -> Result<NoopResponse, HandlerError> {
        Ok(NoopResponse)
    }
}

struct FixedRegistry {
    names: Vec<String>,
}

impl ServiceRegistry for FixedRegistry {
    type Request = NoopRequest;
    type Response = NoopResponse;

    fn list_names(
        &self,
        _req: ListNamesRequest,
    ) -> Result<edge_application_handler::ListNamesResponse, HandlerError> {
        Ok(edge_application_handler::ListNamesResponse {
            names: self.names.clone(),
        })
    }

    fn get(
        &self,
        req: ServiceLookupRequest,
    ) -> Result<ServiceLookupResponse<NoopRequest, NoopResponse>, HandlerError> {
        if self.names.contains(&req.name) {
            Ok(ServiceLookupResponse {
                service: Some(Arc::new(StubService)),
            })
        } else {
            Ok(ServiceLookupResponse { service: None })
        }
    }
}

/// @covers: ServiceRegistry::list_names — returns every registered name
#[test]
fn test_list_names_returns_registered_names_happy() {
    let registry = FixedRegistry {
        names: vec!["svc-1".to_string(), "svc-2".to_string()],
    };
    assert_eq!(
        registry.list_names(ListNamesRequest).unwrap().names,
        vec!["svc-1".to_string(), "svc-2".to_string()]
    );
}

/// @covers: ServiceRegistry::list_names — empty registry yields no names
#[test]
fn test_list_names_empty_registry_returns_empty_error() {
    let registry = FixedRegistry { names: vec![] };
    assert!(registry
        .list_names(ListNamesRequest)
        .unwrap()
        .names
        .is_empty());
}

/// @covers: ServiceRegistry::list_names — a single-entry registry round-trips its own name
#[test]
fn test_list_names_single_entry_matches_get_edge() {
    let registry = FixedRegistry {
        names: vec!["solo".to_string()],
    };
    let names = registry.list_names(ListNamesRequest).unwrap().names;
    assert_eq!(names.len(), 1);
    let looked_up = registry
        .get(ServiceLookupRequest {
            name: names[0].clone(),
        })
        .unwrap();
    assert!(looked_up.service.is_some());
}

/// @covers: ServiceRegistry::get — registered name resolves to an executable service
#[test]
fn test_get_registered_name_returns_executable_service_happy() {
    let registry = FixedRegistry {
        names: vec!["noop".to_string()],
    };
    let resp = registry
        .get(ServiceLookupRequest {
            name: "noop".to_string(),
        })
        .unwrap();
    let service = resp.service.expect("service should be present");
    assert_eq!(block_on(service.execute(NoopRequest)), Ok(NoopResponse));
}

/// @covers: ServiceRegistry::get — unknown name returns no service
#[test]
fn test_get_unknown_name_returns_none_error() {
    let registry = FixedRegistry { names: vec![] };
    let resp = registry
        .get(ServiceLookupRequest {
            name: "missing".to_string(),
        })
        .unwrap();
    assert!(resp.service.is_none());
}

/// @covers: ServiceRegistry::get — empty name never matches a non-empty registry
#[test]
fn test_get_empty_name_against_nonempty_registry_returns_none_edge() {
    let registry = FixedRegistry {
        names: vec!["svc".to_string()],
    };
    let resp = registry
        .get(ServiceLookupRequest {
            name: String::new(),
        })
        .unwrap();
    assert!(resp.service.is_none());
}
