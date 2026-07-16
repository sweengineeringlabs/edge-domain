//! Integration tests for the `Response` trait.

use edge_application_base::Response;

struct FarewellResponse {
    text: String,
}

struct Pong;

impl Response for FarewellResponse {}
impl Response for Pong {}

fn accept_response<T: Response>(resp: T) -> T {
    resp
}

/// @covers: Response — a concrete type satisfies the trait bound and can cross a generic boundary
#[test]
fn test_response_bound_accepts_implementing_type_happy() {
    let resp = accept_response(FarewellResponse {
        text: "hello".to_string(),
    });
    assert_eq!(resp.text, "hello");
}

/// @covers: Response — re-exported trait resolves to the type declared in `api::base::traits`
#[test]
fn test_response_type_name_resolves_via_public_api_error() {
    assert!(std::any::type_name::<FarewellResponse>().ends_with("FarewellResponse"));
}

/// @covers: Response — a zero-sized type satisfies the bound just as well as a type with fields
#[test]
fn test_response_bound_accepts_zero_sized_type_edge() {
    let resp = accept_response(Pong);
    assert_eq!(std::mem::size_of_val(&resp), 0);
}
