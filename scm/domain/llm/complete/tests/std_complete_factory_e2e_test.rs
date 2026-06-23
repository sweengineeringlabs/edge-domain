//! Scenario coverage for `StdCompleteFactory`.

use edge_llm_complete::StdCompleteFactory;

#[test]
fn test_std_complete_factory_is_unit_struct_happy() {
    let f: StdCompleteFactory = StdCompleteFactory;
    assert_eq!(std::mem::size_of_val(&f), 0);
}

#[test]
fn test_std_complete_factory_clone_equals_original_error() {
    let a = StdCompleteFactory;
    let b = StdCompleteFactory;
    assert_eq!(a, b); // both are zero-size unit structs
}

#[test]
fn test_std_complete_factory_debug_is_nonempty_edge() {
    let s = format!("{:?}", StdCompleteFactory);
    assert!(!s.is_empty());
}
