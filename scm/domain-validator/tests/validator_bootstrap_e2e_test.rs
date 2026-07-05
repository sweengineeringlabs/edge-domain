//! End-to-end contract tests for the `ValidatorBootstrap` trait, exercised through
//! the crate's canonical `StdValidatorFactory` implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_validator::{
    AlwaysValid, BootstrapNameRequest, StdValidatorFactory, ValidationRequest, ValidationResponse,
    Validator, ValidatorBootstrap,
};

/// @covers: ValidatorBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_happy() {
    let f = StdValidatorFactory;
    assert_eq!(f.bootstrap_name(BootstrapNameRequest).unwrap().name, "validator");
}

/// @covers: ValidatorBootstrap::always_valid
#[test]
fn test_always_valid_constructs_passing_validator_happy() {
    let v: AlwaysValid = StdValidatorFactory::always_valid();
    assert_eq!(v.validate(ValidationRequest), Ok(ValidationResponse));
}

/// @covers: ValidatorBootstrap::std_factory
#[test]
fn test_std_factory_returns_factory_instance_edge() {
    let f: StdValidatorFactory = StdValidatorFactory::std_factory();
    assert_eq!(std::mem::size_of_val(&f), 0, "StdValidatorFactory must be a zero-sized marker");
}
