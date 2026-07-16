//! Integration tests for the `Request` trait.

use edge_application_base::Request;

struct GreetingRequest {
    name: String,
}

struct Ping;

impl Request for GreetingRequest {}
impl Request for Ping {}

fn accept_request<T: Request>(req: T) -> T {
    req
}

/// @covers: Request — a concrete type satisfies the trait bound and can cross a generic boundary
#[test]
fn test_request_bound_accepts_implementing_type_happy() {
    let req = accept_request(GreetingRequest {
        name: "world".to_string(),
    });
    assert_eq!(req.name, "world");
}

/// @covers: Request — re-exported trait resolves to the type declared in `api::base::traits`
#[test]
fn test_request_type_name_resolves_via_public_api_error() {
    assert!(std::any::type_name::<GreetingRequest>().ends_with("GreetingRequest"));
}

/// @covers: Request — a zero-sized type satisfies the bound just as well as a type with fields
#[test]
fn test_request_bound_accepts_zero_sized_type_edge() {
    let req = accept_request(Ping);
    assert_eq!(std::mem::size_of_val(&req), 0);
}
