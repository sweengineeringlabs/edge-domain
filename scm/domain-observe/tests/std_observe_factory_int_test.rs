use edge_domain_observe::StdObserveFactory;

#[test]
fn test_std_observe_factory_is_constructible_happy() {
    let _ = StdObserveFactory;
}

#[test]
fn test_std_observe_factory_two_instances_are_distinct_error() {
    let _a = StdObserveFactory;
    let _b = StdObserveFactory;
}

#[test]
fn test_std_observe_factory_unit_struct_zero_size_edge() {
    assert_eq!(std::mem::size_of::<StdObserveFactory>(), 0);
}
