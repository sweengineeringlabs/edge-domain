//! Integration tests — `step_svc_factory` constants.
//! @covers STEP_SVC, STEP_SVC_FACTORY
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_pipeline::{STEP_SVC, STEP_SVC_FACTORY};

/// @covers: STEP_SVC
#[test]
fn test_step_svc_name_is_stable_happy() {
    assert_eq!(STEP_SVC, "step");
}

/// @covers: STEP_SVC_FACTORY
#[test]
fn test_step_svc_factory_name_is_stable_happy() {
    assert_eq!(STEP_SVC_FACTORY, "step_svc_factory");
}

/// @covers: STEP_SVC
#[test]
fn test_step_svc_is_non_empty_error() {
    assert!(!STEP_SVC.is_empty());
}

/// @covers: STEP_SVC_FACTORY
#[test]
fn test_step_svc_factory_is_non_empty_error() {
    assert!(!STEP_SVC_FACTORY.is_empty());
}

/// @covers: STEP_SVC, STEP_SVC_FACTORY
#[test]
fn test_step_svc_and_factory_names_differ_edge() {
    assert_ne!(STEP_SVC, STEP_SVC_FACTORY);
}
