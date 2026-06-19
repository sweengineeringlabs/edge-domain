#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `Agent` constructed via `saf::noop_agent`.

use edge_llm_agent::{Agent, NoopAgent, Role, ToolChoice};
use edge_llm_provider::Provider;

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

/// @covers: NoopAgent::provider
#[test]
fn test_noop_agent_provider_returns_arc_dyn_provider_happy() {
    use std::sync::Arc;
    let _p: Arc<dyn Provider> = NoopAgent.provider();
}

/// @covers: NoopAgent::provider
#[test]
fn test_noop_agent_provider_health_check_ok_happy() {
    assert!(NoopAgent.provider().health_check().is_ok());
}

/// @covers: NoopAgent::provider
#[test]
fn test_noop_agent_provider_completer_accessible_edge() {
    use std::sync::Arc;
    let p = NoopAgent.provider();
    let c1 = p.completer();
    let c2 = p.completer();
    assert!(Arc::ptr_eq(&c1, &c2));
}
