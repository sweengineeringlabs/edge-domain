//! Tests for [`EmptinessResponse`] — emptiness wrapper response.

use edge_application_service::EmptinessResponse;

/// @covers: EmptinessResponse — empty registry
#[test]
fn test_emptiness_response_empty_happy() {
    let resp = EmptinessResponse { empty: true };
    assert!(resp.empty);
}

/// @covers: EmptinessResponse — non-empty registry
#[test]
fn test_emptiness_response_not_empty_happy() {
    let resp = EmptinessResponse { empty: false };
    assert!(!resp.empty);
}

/// @covers: EmptinessResponse — state transitions
#[test]
fn test_emptiness_response_state_transition_edge() {
    let empty = EmptinessResponse { empty: true };
    let not_empty = EmptinessResponse { empty: false };
    assert_ne!(empty.empty, not_empty.empty);
}
