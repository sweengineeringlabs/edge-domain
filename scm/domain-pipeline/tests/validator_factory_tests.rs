use edge_domain_pipeline::{ValidatorFactory, Validator};

/// @covers ValidatorFactory::create - enabled
#[test]
fn test_validator_factory_create_happy_enabled() {
    let validator = ValidatorFactory::create(true);
    assert!(validator.is_enabled());
}

/// @covers ValidatorFactory::create - disabled
#[test]
fn test_validator_factory_create_happy_disabled() {
    let validator = ValidatorFactory::create(false);
    assert!(!validator.is_enabled());
}
