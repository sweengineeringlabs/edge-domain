use edge_domain_pipeline::create_validator;

#[test]
fn test_validator_factory_create_happy_enabled() {
    let validator = create_validator(true);
    assert!(validator.is_enabled());
}

#[test]
fn test_validator_factory_create_happy_disabled() {
    let validator = create_validator(false);
    assert!(!validator.is_enabled());
}
