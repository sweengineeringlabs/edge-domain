use edge_domain_validator::{StdValidatorFactory, ValidatorBootstrap, Validator};

#[test]
fn test_std_factory_always_valid_accepts_any_input_happy() {
    let v = StdValidatorFactory::always_valid();
    assert_eq!(v.validate(), Ok(()));
}

#[test]
fn test_std_factory_always_valid_never_errors_happy() {
    let v = StdValidatorFactory::always_valid();
    assert_eq!(v.validate(), Ok(()));
}

#[test]
fn test_std_factory_std_factory_returns_copy_instance_edge() {
    let f = StdValidatorFactory::std_factory();
    let _f2 = f;
    let _f3 = f; // Copy — usable after move
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<StdValidatorFactory>());
}
