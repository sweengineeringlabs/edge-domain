//! Tests for [`ServiceLookupRequest`] — service lookup request.

use edge_application_service::ServiceLookupRequest;

/// @covers: ServiceLookupRequest — constructible with name
#[test]
fn test_service_lookup_request_new_happy() {
    let req = ServiceLookupRequest {
        name: "test".to_string(),
    };
    assert_eq!(req.name, "test");
}

/// @covers: ServiceLookupRequest — empty name allowed
#[test]
fn test_service_lookup_request_empty_name_happy() {
    let req = ServiceLookupRequest {
        name: "".to_string(),
    };
    assert_eq!(req.name, "");
}

/// @covers: ServiceLookupRequest — long name
#[test]
fn test_service_lookup_request_long_name_edge() {
    let long_name = "service".repeat(100);
    let req = ServiceLookupRequest {
        name: long_name.clone(),
    };
    assert_eq!(req.name, long_name);
}
