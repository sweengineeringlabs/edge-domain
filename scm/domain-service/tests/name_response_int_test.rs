//! Tests for [`NameResponse`] — name wrapper response.

use edge_domain_service::NameResponse;

/// @covers: NameResponse — constructible with name
#[test]
fn test_name_response_new_happy() {
    let resp = NameResponse {
        name: "test".to_string(),
    };
    assert_eq!(resp.name, "test");
}

/// @covers: NameResponse — empty name allowed
#[test]
fn test_name_response_empty_name_happy() {
    let resp = NameResponse {
        name: "".to_string(),
    };
    assert_eq!(resp.name, "");
}

/// @covers: NameResponse — long name
#[test]
fn test_name_response_long_name_edge() {
    let long_name = "a".repeat(1000);
    let resp = NameResponse {
        name: long_name.clone(),
    };
    assert_eq!(resp.name, long_name);
}
