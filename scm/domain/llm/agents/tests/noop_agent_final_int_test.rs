#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `Agent` constructed via `saf::noop_agent`.

use edge_llm_agent::{Agent, NoopAgent, Role, ToolChoice};

#[test]
fn test_noop_agent_id_is_noop() {
    assert_eq!(NoopAgent.id(), "noop");
}

#[test]
fn test_noop_agent_exposes_no_skills() {
    assert!(NoopAgent.skills().is_empty());
}

#[test]
fn test_noop_agent_execute_skill_reports_missing_skill() {
    let result = futures::executor::block_on(NoopAgent.execute_skill("x", "{}".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_noop_agent_default_role_is_assistant() {
    assert_eq!(NoopAgent.supported_role(), Role::Assistant);
}

#[test]
fn test_noop_agent_default_tool_choice_is_auto() {
    assert_eq!(NoopAgent.tool_choice(), ToolChoice::Auto);
}
