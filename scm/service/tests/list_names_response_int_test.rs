//! Tests for [`ListNamesResponse`] — names wrapper response.

use edge_application_service::ListNamesResponse;

/// @covers: ListNamesResponse — constructible with names
#[test]
fn test_list_names_response_new_empty_happy() {
    let resp = ListNamesResponse { names: vec![] };
    assert!(resp.names.is_empty());
}

/// @covers: ListNamesResponse — multiple names
#[test]
fn test_list_names_response_with_names_happy() {
    let resp = ListNamesResponse {
        names: vec!["a".to_string(), "b".to_string()],
    };
    assert_eq!(resp.names.len(), 2);
}

/// @covers: ListNamesResponse — many names
#[test]
fn test_list_names_response_many_names_edge() {
    let names: Vec<String> = (0..100).map(|i| format!("name{}", i)).collect();
    let resp = ListNamesResponse {
        names: names.clone(),
    };
    assert_eq!(resp.names.len(), 100);
    assert_eq!(resp.names, names);
}
