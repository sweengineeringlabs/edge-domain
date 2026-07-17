//! Tests for [`LenResponse`] — length wrapper response.

use edge_application_service::LenResponse;

/// @covers: LenResponse — constructible with count
#[test]
fn test_len_response_new_zero_happy() {
    let resp = LenResponse { count: 0 };
    assert_eq!(resp.count, 0);
}

/// @covers: LenResponse — nonzero count
#[test]
fn test_len_response_nonzero_count_happy() {
    let resp = LenResponse { count: 42 };
    assert_eq!(resp.count, 42);
}

/// @covers: LenResponse — large count
#[test]
fn test_len_response_large_count_edge() {
    let resp = LenResponse { count: usize::MAX };
    assert_eq!(resp.count, usize::MAX);
}
