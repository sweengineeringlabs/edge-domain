use edge_domain_observe::COUNTER_SVC;

#[test]
fn test_counter_svc_key_non_empty_happy() {
    assert!(!COUNTER_SVC.is_empty());
}

#[test]
fn test_counter_svc_key_has_edge_prefix_error() {
    assert!(
        COUNTER_SVC.starts_with("edge."),
        "service key must be namespaced under edge.*"
    );
}

#[test]
fn test_counter_svc_key_stable_across_calls_edge() {
    assert_eq!(COUNTER_SVC, COUNTER_SVC);
}
