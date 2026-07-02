#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{EnablementRequest, ValidatorSvc};

#[test]
fn test_validator_factory_create_happy_enabled() {
    let validator = ValidatorSvc::create(true);
    assert!(
        validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}

#[test]
fn test_validator_factory_create_happy_disabled() {
    let validator = ValidatorSvc::create(false);
    assert!(
        !validator
            .is_enabled(EnablementRequest)
            .expect("must succeed")
            .enabled
    );
}
