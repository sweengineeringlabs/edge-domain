//! Coverage for api/validator/types/validator_default.rs
use edge_domain::ValidatorDefault;

#[test]
fn test_validator_default_is_constructible_happy() {
    let _v = ValidatorDefault;
}

#[test]
fn test_validator_default_zst_size_edge() {
    assert_eq!(std::mem::size_of::<ValidatorDefault>(), 0);
}

#[test]
fn test_validator_default_two_instances_are_independent_edge() {
    let _a = ValidatorDefault;
    let _b = ValidatorDefault;
}
