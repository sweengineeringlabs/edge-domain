//! Scenario coverage for the `tool_call_loop_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_llm_complete::{
    BoundedToolCallLoop, CompletionRequest, NoopCompleter, ToolCallLoop, ToolCallLoopRequest,
    TOOL_CALL_LOOP_SVC,
};

#[test]
fn test_tool_call_loop_svc_constant_is_expected_value_happy() {
    assert_eq!(TOOL_CALL_LOOP_SVC, "tool_call_loop");
}

#[test]
fn test_tool_call_loop_svc_constant_is_nonempty_error() {
    assert!(!TOOL_CALL_LOOP_SVC.is_empty());
}

#[tokio::test]
async fn test_tool_call_loop_svc_facade_reaches_terminal_response_edge() {
    let l = BoundedToolCallLoop::new(Arc::new(NoopCompleter), Arc::new(NoopCompleter));
    let request = CompletionRequest::new("test-model", vec![]);
    let result = l
        .run(ToolCallLoopRequest {
            request: &request,
            max_turns: 1,
        })
        .await;
    assert!(result.is_err());
}
