//! Layer-level e2e coverage for the `CompleteBootstrap` trait via a test-double implementer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::*;

struct BootstrapDouble;

impl CompleteBootstrap for BootstrapDouble {}

/// @covers: CompleteBootstrap::bootstrap_name — default impl reports "complete"
#[test]
fn test_bootstrap_name_default_reports_complete_happy() {
    let name = BootstrapDouble
        .bootstrap_name(CompleteBootstrapNameRequest)
        .expect("bootstrap_name ok")
        .name;
    assert_eq!(name, "complete");
}

/// @covers: CompleteBootstrap::echo_completer — builds a working EchoCompleter
#[test]
fn test_echo_completer_builds_working_instance_happy() {
    let completer = BootstrapDouble::echo_completer();
    assert_eq!(completer, EchoCompleter);
}

/// @covers: CompleteBootstrap::noop_completer — builds a working NoopCompleter
#[test]
fn test_noop_completer_builds_working_instance_happy() {
    let completer = BootstrapDouble::noop_completer();
    assert_eq!(completer, NoopCompleter);
}

/// @covers: CompleteBootstrap::user_message — builds a user-role message
#[test]
fn test_user_message_sets_user_role_happy() {
    let msg = BootstrapDouble::user_message("hi".to_string());
    assert_eq!(msg.role, Role::User);
}

/// @covers: CompleteBootstrap::request — builds a request from model and messages
#[test]
fn test_request_sets_model_and_messages_edge() {
    let req = BootstrapDouble::request(
        "echo".to_string(),
        vec![BootstrapDouble::user_message("hi".to_string())],
    );
    assert_eq!(req.model, "echo");
    assert_eq!(req.messages.len(), 1);
}

/// @covers: CompleteBootstrap::token_usage — builds a zeroed usage record
#[test]
fn test_token_usage_defaults_to_zero_error() {
    let usage = BootstrapDouble::token_usage();
    assert_eq!(usage.total_tokens, 0);
}
