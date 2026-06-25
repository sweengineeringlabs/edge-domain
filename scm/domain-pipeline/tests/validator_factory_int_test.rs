use edge_domain_pipeline::ValidatorSvc;

#[test]
fn test_validator_factory_create_happy_enabled() {
    let validator = ValidatorSvc::create(true);
    assert!(validator.is_enabled());
}

#[test]
fn test_validator_factory_create_happy_disabled() {
    let validator = ValidatorSvc::create(false);
    assert!(!validator.is_enabled());
}
