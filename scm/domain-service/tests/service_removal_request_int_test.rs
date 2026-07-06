//! Tests for [`ServiceRemovalRequest`] — service removal request.

use edge_domain_service::ServiceRemovalRequest;

/// @covers: ServiceRemovalRequest — constructible with name
#[test]
fn test_service_removal_request_new_happy() {
    let req = ServiceRemovalRequest {
        name: "test".to_string(),
    };
    assert_eq!(req.name, "test");
}

/// @covers: ServiceRemovalRequest — empty name allowed
#[test]
fn test_service_removal_request_empty_name_happy() {
    let req = ServiceRemovalRequest {
        name: "".to_string(),
    };
    assert_eq!(req.name, "");
}

/// @covers: ServiceRemovalRequest — long name
#[test]
fn test_service_removal_request_long_name_edge() {
    let long_name = "x".repeat(1000);
    let req = ServiceRemovalRequest {
        name: long_name.clone(),
    };
    assert_eq!(req.name, long_name);
}
