//! SAF facade tests — `Policy` trait.

use edge_domain_policy::{Policy, PolicyViolation};

struct NonZero;
impl Policy for NonZero {
    type Input = u64;
    fn name(&self) -> &'static str {
        "non-zero"
    }
    fn evaluate(&self, v: &u64) -> Result<(), PolicyViolation> {
        if *v > 0 {
            Ok(())
        } else {
            Err(PolicyViolation::new("non-zero", "must be > 0"))
        }
    }
}

/// @covers: Policy::name
#[test]
fn test_name_returns_static_label_happy() {
    assert_eq!(NonZero.name(), "non-zero");
}

/// @covers: Policy::name — stable across instances
#[test]
fn test_name_is_stable_across_calls_error() {
    let p = NonZero;
    assert_eq!(p.name(), p.name());
}

/// @covers: Policy::name — usable via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_edge() {
    let p: &dyn Policy<Input = u64> = &NonZero;
    assert_eq!(p.name(), "non-zero");
}

/// @covers: Policy::evaluate — satisfied rule returns Ok
#[test]
fn test_evaluate_satisfied_rule_returns_ok_happy() {
    assert!(NonZero.evaluate(&5).is_ok());
}

/// @covers: Policy::evaluate — violated rule returns Err
#[test]
fn test_evaluate_violated_rule_returns_err_error() {
    let err = NonZero.evaluate(&0);
    assert!(err.is_err());
}

/// @covers: Policy::evaluate — boundary at zero
#[test]
fn test_evaluate_boundary_value_edge() {
    assert!(NonZero.evaluate(&1).is_ok());
    assert!(NonZero.evaluate(&0).is_err());
}
