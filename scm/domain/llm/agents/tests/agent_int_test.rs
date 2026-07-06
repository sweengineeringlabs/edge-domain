#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests — `Agent` trait.

use async_trait::async_trait;
use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
use edge_domain_handler::HandlerContext;
use edge_domain_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use edge_llm_agent::{
    Agent, AgentDescriptionRequest, AgentError, AgentIdRequest, AgentNameRequest,
    AgentProviderRequest, AgentSkillsRequest, MessageBuilderRequest, MessageContent,
    MessageSendRequest, Role, SkillExecutionRequest, SkillLookupRequest, SupportedRoleRequest,
    ToolChoice, ToolChoicePreferenceRequest,
};
use edge_llm_provider::{
    CompleterRequest, EchoProviderCompleter, HealthCheckRequest, ModelInfo, Provider,
    ProviderBootstrap, ProviderConfig, StdProviderFactory,
};
use std::sync::Arc;

fn noop_provider() -> Arc<dyn Provider> {
    StdProviderFactory::provider(
        ProviderConfig::new("noop".to_string(), 0.0, 0),
        Box::<ModelInfo>::default(),
        Arc::new(EchoProviderCompleter),
        StdObserveFactory::noop_arc_observe_context(),
    )
}

struct SuccessAgent;

#[async_trait]
impl Agent for SuccessAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<edge_llm_agent::AgentIdResponse, AgentError> {
        Ok(edge_llm_agent::AgentIdResponse {
            id: "success".to_string(),
        })
    }

    fn name(
        &self,
        _req: AgentNameRequest,
    ) -> Result<edge_llm_agent::AgentNameResponse, AgentError> {
        Ok(edge_llm_agent::AgentNameResponse {
            name: "Success Agent".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<edge_llm_agent::AgentDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::AgentDescriptionResponse {
            description: "Always succeeds".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        req: SkillExecutionRequest<'_>,
    ) -> Result<edge_llm_agent::SkillExecutionResponse, AgentError> {
        Ok(edge_llm_agent::SkillExecutionResponse {
            output: format!("{}:{}", req.skill_name, req.input),
        })
    }

    fn skills(
        &self,
        _req: AgentSkillsRequest,
    ) -> Result<edge_llm_agent::AgentSkillsResponse, AgentError> {
        Ok(edge_llm_agent::AgentSkillsResponse { skills: vec![] })
    }

    fn provider(
        &self,
        _req: AgentProviderRequest,
    ) -> Result<edge_llm_agent::AgentProviderResponse, AgentError> {
        Ok(edge_llm_agent::AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

struct FailingAgent;

#[async_trait]
impl Agent for FailingAgent {
    fn id(&self, _req: AgentIdRequest) -> Result<edge_llm_agent::AgentIdResponse, AgentError> {
        Ok(edge_llm_agent::AgentIdResponse {
            id: "failing".to_string(),
        })
    }

    fn name(
        &self,
        _req: AgentNameRequest,
    ) -> Result<edge_llm_agent::AgentNameResponse, AgentError> {
        Ok(edge_llm_agent::AgentNameResponse {
            name: "Failing Agent".to_string(),
        })
    }

    fn description(
        &self,
        _req: AgentDescriptionRequest,
    ) -> Result<edge_llm_agent::AgentDescriptionResponse, AgentError> {
        Ok(edge_llm_agent::AgentDescriptionResponse {
            description: "Always fails".to_string(),
        })
    }

    async fn execute_skill(
        &self,
        _req: SkillExecutionRequest<'_>,
    ) -> Result<edge_llm_agent::SkillExecutionResponse, AgentError> {
        Err(AgentError::ExecutionFailed(
            "deliberate failure".to_string(),
        ))
    }

    fn skills(
        &self,
        _req: AgentSkillsRequest,
    ) -> Result<edge_llm_agent::AgentSkillsResponse, AgentError> {
        Ok(edge_llm_agent::AgentSkillsResponse { skills: vec![] })
    }

    fn provider(
        &self,
        _req: AgentProviderRequest,
    ) -> Result<edge_llm_agent::AgentProviderResponse, AgentError> {
        Ok(edge_llm_agent::AgentProviderResponse {
            provider: noop_provider(),
        })
    }
}

/// @covers: Agent::id
#[test]
fn test_trait_agent_happy_id_returns_configured_id() {
    assert_eq!(SuccessAgent.id(AgentIdRequest).unwrap().id, "success");
}

/// @covers: Agent::id — multiple implementations
#[test]
fn test_trait_agent_happy_id_differs_by_implementation() {
    assert_ne!(
        SuccessAgent.id(AgentIdRequest).unwrap().id,
        FailingAgent.id(AgentIdRequest).unwrap().id
    );
}

/// @covers: Agent::name
#[test]
fn test_trait_agent_happy_name_returns_configured_name() {
    assert_eq!(
        SuccessAgent.name(AgentNameRequest).unwrap().name,
        "Success Agent"
    );
}

/// @covers: Agent::description
#[test]
fn test_trait_agent_happy_description_returns_configured_description() {
    assert_eq!(
        SuccessAgent
            .description(AgentDescriptionRequest)
            .unwrap()
            .description,
        "Always succeeds"
    );
}

/// @covers: Agent::execute_skill — success case
#[test]
fn test_trait_agent_happy_execute_skill_success_returns_ok_response() {
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(SuccessAgent.execute_skill(SkillExecutionRequest {
        skill_name: "analyze",
        input: "test_input".to_string(),
        ctx,
    }));
    assert!(result.is_ok());
    assert_eq!(result.unwrap().output, "analyze:test_input");
}

/// @covers: Agent::execute_skill — failure case
#[test]
fn test_trait_agent_error_execute_skill_failure_returns_execution_failed() {
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(FailingAgent.execute_skill(SkillExecutionRequest {
        skill_name: "any_skill",
        input: "input".to_string(),
        ctx,
    }));
    assert!(result.is_err());
    match result {
        Err(AgentError::ExecutionFailed(msg)) => {
            assert_eq!(msg, "deliberate failure");
        }
        _ => panic!("Expected ExecutionFailed error"),
    }
}

/// @covers: Agent::execute_skill — input is passed through
#[test]
fn test_trait_agent_happy_execute_skill_preserves_input() {
    let security = SecurityContext::unauthenticated();
    let commands = StdCommandBusFactory::direct();
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let result = futures::executor::block_on(SuccessAgent.execute_skill(SkillExecutionRequest {
        skill_name: "skill",
        input: "preserved".to_string(),
        ctx,
    }));
    assert_eq!(result.unwrap().output, "skill:preserved");
}

/// @covers: Agent::skills — empty implementation
#[test]
fn test_trait_agent_edge_skills_returns_empty_list() {
    assert_eq!(
        SuccessAgent
            .skills(AgentSkillsRequest)
            .unwrap()
            .skills
            .len(),
        0
    );
}

/// @covers: Agent::skill — delegates to skills()
#[test]
fn test_trait_agent_error_skill_returns_skill_not_found_when_empty() {
    let result = SuccessAgent.skill(SkillLookupRequest {
        name: "nonexistent",
    });
    assert!(matches!(result, Err(AgentError::SkillNotFound(_))));
}

/// @covers: Agent — all methods work together
#[test]
fn test_trait_agent_happy_all_methods_together_consistent() {
    assert!(!SuccessAgent.id(AgentIdRequest).unwrap().id.is_empty());
    assert!(!SuccessAgent.name(AgentNameRequest).unwrap().name.is_empty());
    assert!(!SuccessAgent
        .description(AgentDescriptionRequest)
        .unwrap()
        .description
        .is_empty());
    assert_eq!(SuccessAgent.id(AgentIdRequest).unwrap().id, "success");
}

// --- send ---

/// @covers: send
#[test]
fn test_send_accepts_message_happy() {
    let agent = SuccessAgent;
    let msg = agent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .content("hello")
        .build();
    let delivered = agent
        .send(MessageSendRequest {
            message: Box::new(msg),
        })
        .unwrap()
        .delivered;
    assert_eq!(delivered, 1);
}

/// @covers: send
#[test]
fn test_send_default_is_stateless_error() {
    // The default impl does not accumulate: each send reports a single message.
    let agent = SuccessAgent;
    let msg_a = agent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .content("a")
        .build();
    let msg_b = agent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .content("b")
        .build();
    let first = agent
        .send(MessageSendRequest {
            message: Box::new(msg_a),
        })
        .unwrap()
        .delivered;
    let second = agent
        .send(MessageSendRequest {
            message: Box::new(msg_b),
        })
        .unwrap()
        .delivered;
    assert_eq!(first, second);
}

/// @covers: send
#[test]
fn test_send_empty_content_edge() {
    let agent = SuccessAgent;
    let msg = agent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .build();
    let delivered = agent
        .send(MessageSendRequest {
            message: Box::new(msg),
        })
        .unwrap()
        .delivered;
    assert_eq!(delivered, 1);
}

// --- supported_role ---

/// @covers: supported_role
#[test]
fn test_supported_role_defaults_assistant_happy() {
    assert_eq!(
        SuccessAgent
            .supported_role(SupportedRoleRequest)
            .unwrap()
            .role,
        Role::Assistant
    );
}

/// @covers: supported_role
#[test]
fn test_supported_role_is_not_user_error() {
    assert_ne!(
        SuccessAgent
            .supported_role(SupportedRoleRequest)
            .unwrap()
            .role,
        Role::User
    );
}

/// @covers: supported_role
#[test]
fn test_supported_role_consistent_across_impls_edge() {
    assert_eq!(
        SuccessAgent
            .supported_role(SupportedRoleRequest)
            .unwrap()
            .role,
        FailingAgent
            .supported_role(SupportedRoleRequest)
            .unwrap()
            .role
    );
}

// --- tool_choice ---

/// @covers: tool_choice
#[test]
fn test_tool_choice_defaults_auto_happy() {
    assert_eq!(
        SuccessAgent
            .tool_choice(ToolChoicePreferenceRequest)
            .unwrap()
            .choice,
        ToolChoice::Auto
    );
}

/// @covers: tool_choice
#[test]
fn test_tool_choice_is_not_none_error() {
    assert_ne!(
        SuccessAgent
            .tool_choice(ToolChoicePreferenceRequest)
            .unwrap()
            .choice,
        ToolChoice::None
    );
}

/// @covers: tool_choice
#[test]
fn test_tool_choice_consistent_across_impls_edge() {
    assert_eq!(
        SuccessAgent
            .tool_choice(ToolChoicePreferenceRequest)
            .unwrap()
            .choice,
        FailingAgent
            .tool_choice(ToolChoicePreferenceRequest)
            .unwrap()
            .choice
    );
}

// --- message_builder ---

/// @covers: message_builder
#[test]
fn test_message_builder_builds_message_happy() {
    let msg = SuccessAgent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .content("hi")
        .build();
    assert_eq!(msg.role, Role::User);
}

/// @covers: message_builder
#[test]
fn test_message_builder_role_override_error() {
    let msg = SuccessAgent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .role(Role::System)
        .content("sys")
        .build();
    assert_ne!(msg.role, Role::User);
}

/// @covers: message_builder
#[test]
fn test_message_builder_default_content_empty_edge() {
    let msg = SuccessAgent
        .message_builder(MessageBuilderRequest)
        .unwrap()
        .builder
        .build();
    assert_eq!(msg.content, MessageContent::text(""));
}

// --- provider ---

/// @covers: Agent::provider
#[test]
fn test_provider_returns_arc_dyn_provider_happy() {
    let _p: Arc<dyn Provider> = SuccessAgent
        .provider(AgentProviderRequest)
        .unwrap()
        .provider;
}

/// @covers: Agent::provider
#[test]
fn test_provider_health_check_ok_happy() {
    let result = SuccessAgent
        .provider(AgentProviderRequest)
        .unwrap()
        .provider
        .health_check(HealthCheckRequest);
    assert!(result.is_ok());
}

/// @covers: Agent::provider
#[test]
fn test_provider_distinct_per_impl_error() {
    // Two different agent types may back different providers.
    let p1 = SuccessAgent
        .provider(AgentProviderRequest)
        .unwrap()
        .provider;
    let p2 = FailingAgent
        .provider(AgentProviderRequest)
        .unwrap()
        .provider;
    // Both healthy — confirming each returns a usable provider.
    assert!(p1.health_check(HealthCheckRequest).is_ok());
    assert!(p2.health_check(HealthCheckRequest).is_ok());
}

/// @covers: Agent::provider
#[test]
fn test_provider_completer_accessible_from_provider_edge() {
    // Completer is reachable through the provider seam without naming ProviderCore.
    let p = SuccessAgent
        .provider(AgentProviderRequest)
        .unwrap()
        .provider;
    let completer = p.completer(CompleterRequest);
    assert!(completer.is_ok(), "completer must be accessible");
}
