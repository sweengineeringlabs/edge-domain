use edge_domain_validator::{StdValidatorFactory, Validator, ValidatorFactory};

#[test]
fn test_std_factory_always_valid_accepts_any_input_happy() {
    let v = StdValidatorFactory::always_valid();
    assert!(v.validate(&"anything".to_string()).is_ok());
}

#[test]
fn test_std_factory_always_valid_never_errors_error() {
    let v = StdValidatorFactory::always_valid();
    assert!(v.validate(&String::new()).is_ok());
}

#[test]
fn test_std_factory_std_factory_returns_copy_instance_edge() {
    let f = StdValidatorFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after move
}
