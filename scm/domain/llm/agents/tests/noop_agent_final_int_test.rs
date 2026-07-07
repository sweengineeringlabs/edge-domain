#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `Agent` constructed via `saf::noop_agent`.

use edge_domain_command::DirectCommandBus;
use edge_domain_handler::HandlerContext;
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_agent::{
    Agent, AgentIdRequest, AgentProviderRequest, AgentSkillsRequest, NoopAgent, Role,
    SkillExecutionRequest, SupportedRoleRequest, ToolChoice, ToolChoicePreferenceRequest,
};
use edge_llm_provider::{CompleterRequest, HealthCheckRequest, Provider};

#[test]
fn test_noop_agent_id_is_noop() {
    assert_eq!(NoopAgent.id(AgentIdRequest).unwrap().id, "noop");
}

#[test]
fn test_noop_agent_exposes_no_skills() {
    assert!(NoopAgent
        .skills(AgentSkillsRequest)
        .unwrap()
        .skills
        .is_empty());
}

#[test]
fn test_noop_agent_execute_skill_reports_missing_skill() {
    let security = SecurityContext::unauthenticated();
    let commands = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(NoopAgent.execute_skill(SkillExecutionRequest {
        skill_name: "x",
        input: "{}".to_string(),
        ctx,
    }));
    assert!(result.is_err());
}

#[test]
fn test_noop_agent_default_role_is_assistant() {
    assert_eq!(
        NoopAgent.supported_role(SupportedRoleRequest).unwrap().role,
        Role::Assistant
    );
}

#[test]
fn test_noop_agent_default_tool_choice_is_auto() {
    assert_eq!(
        NoopAgent
            .tool_choice(ToolChoicePreferenceRequest)
            .unwrap()
            .choice,
        ToolChoice::Auto
    );
}

/// @covers: NoopAgent::provider
#[test]
fn test_noop_agent_provider_returns_arc_dyn_provider_happy() {
    use std::sync::Arc;
    let _p: Arc<dyn Provider> = NoopAgent.provider(AgentProviderRequest).unwrap().provider;
}

/// @covers: NoopAgent::provider
#[test]
fn test_noop_agent_provider_health_check_ok_happy() {
    let result = NoopAgent
        .provider(AgentProviderRequest)
        .unwrap()
        .provider
        .health_check(HealthCheckRequest);
    assert!(result.is_ok());
}

/// @covers: NoopAgent::provider
#[test]
fn test_noop_agent_provider_completer_accessible_edge() {
    use std::sync::Arc;
    let p = NoopAgent.provider(AgentProviderRequest).unwrap().provider;
    let c1 = p.completer(CompleterRequest).unwrap().completer;
    let c2 = p.completer(CompleterRequest).unwrap().completer;
    assert!(Arc::ptr_eq(&c1, &c2));
}
