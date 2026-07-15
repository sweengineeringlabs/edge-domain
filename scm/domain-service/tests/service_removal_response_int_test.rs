//! Tests for [`ServiceRemovalResponse`] — service removal response.

use edge_application_service::ServiceRemovalResponse;

/// @covers: ServiceRemovalResponse — service was present
#[test]
fn test_service_removal_response_was_present_happy() {
    let resp = ServiceRemovalResponse { was_present: true };
    assert!(resp.was_present);
}

/// @covers: ServiceRemovalResponse — service was not present
#[test]
fn test_service_removal_response_was_not_present_happy() {
    let resp = ServiceRemovalResponse { was_present: false };
    assert!(!resp.was_present);
}

/// @covers: ServiceRemovalResponse — transitions between states
#[test]
fn test_service_removal_response_state_transition_edge() {
    let removed = ServiceRemovalResponse { was_present: true };
    let not_found = ServiceRemovalResponse { was_present: false };
    assert_ne!(removed.was_present, not_found.was_present);
}
