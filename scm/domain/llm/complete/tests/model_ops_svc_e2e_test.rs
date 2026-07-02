//! Scenario coverage for the `model_ops_svc` SAF surface.

use edge_llm_complete::{ModelOps, NoopCompleter, MODEL_OPS_SVC};

#[test]
fn test_model_ops_svc_constant_is_expected_value_happy() {
    assert_eq!(MODEL_OPS_SVC, "model_ops");
}

#[test]
fn test_model_ops_svc_constant_is_nonempty_error() {
    assert!(!MODEL_OPS_SVC.is_empty());
}

#[test]
fn test_model_ops_create_model_info_sets_context_window_edge() {
    let info = NoopCompleter::create_model_info("m", "M", "p", 4096u32);
    assert_eq!(info.context_window, 4096);
}
