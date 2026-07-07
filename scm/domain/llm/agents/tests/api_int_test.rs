//! Layer-level coverage for small api/ Request/Response marker types that don't
//! warrant their own dedicated test file — see SEA §5 Option C.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_agent::*;
use std::sync::Arc;

#[test]
fn test_abort_request_constructs() {
    let req = AbortRequest;
    assert_eq!(req, AbortRequest);
}

#[test]
fn test_agent_creation_request_response_construct() {
    let req = AgentCreationRequest {
        id: "a1",
        name: "Agent One",
        description: "desc",
        provider: NoopAgent.provider(AgentProviderRequest).unwrap().provider,
        skills: vec![],
    };
    assert_eq!(req.id, "a1");
    let resp = AgentCreationResponse {
        agent: Arc::new(NoopAgent),
    };
    assert_eq!(resp.agent.id(AgentIdRequest).unwrap().id, "noop");
}

#[test]
fn test_agent_description_request_response_construct() {
    let _req = AgentDescriptionRequest;
    let resp = AgentDescriptionResponse {
        description: "desc".to_string(),
    };
    assert_eq!(resp.description, "desc");
}

#[test]
fn test_agent_handler_request_response_construct() {
    let req = AgentHandlerRequest { skill: "review" };
    assert_eq!(req.skill, "review");
    let handler = NoopAgentManager
        .agent_handler(AgentHandlerRequest { skill: "review" })
        .unwrap()
        .handler;
    let _resp = AgentHandlerResponse { handler };
}

#[test]
fn test_agent_id_request_response_construct() {
    let _req = AgentIdRequest;
    let resp = AgentIdResponse {
        id: "a1".to_string(),
    };
    assert_eq!(resp.id, "a1");
}

#[test]
fn test_agent_id_validation_request_constructs() {
    let req = AgentIdValidationRequest { agent_id: "a1" };
    assert_eq!(req.agent_id, "a1");
}

#[test]
fn test_agent_load_request_response_construct() {
    let req = AgentLoadRequest { spec: "spec.yaml" };
    assert_eq!(req.spec, "spec.yaml");
    let resp = AgentLoadResponse {
        agent: Arc::new(NoopAgent),
    };
    assert_eq!(resp.agent.id(AgentIdRequest).unwrap().id, "noop");
}

#[test]
fn test_agent_lookup_request_response_construct() {
    let req = AgentLookupRequest { id: "a1" };
    assert_eq!(req.id, "a1");
    let resp = AgentLookupResponse {
        agent: Arc::new(NoopAgent),
    };
    assert_eq!(resp.agent.id(AgentIdRequest).unwrap().id, "noop");
}

#[test]
fn test_agent_metadata_builder_request_response_construct() {
    let _req = AgentMetadataBuilderRequest;
    let resp = AgentMetadataBuilderResponse {
        builder: Box::new(AgentMetadataBuilder::new()),
    };
    assert!(resp.builder.build().id.is_empty());
}

#[test]
fn test_agent_metadata_lookup_request_response_construct() {
    let req = AgentMetadataLookupRequest { id: "a1" };
    assert_eq!(req.id, "a1");
    let resp = AgentMetadataLookupResponse {
        metadata: Box::new(AgentMetadata::default()),
    };
    assert_eq!(resp.metadata.id, "");
}

#[test]
fn test_agent_name_request_response_construct() {
    let _req = AgentNameRequest;
    let resp = AgentNameResponse {
        name: "Agent".to_string(),
    };
    assert_eq!(resp.name, "Agent");
}

#[test]
fn test_agent_provider_request_response_construct() {
    let _req = AgentProviderRequest;
    let resp = NoopAgent.provider(AgentProviderRequest).unwrap();
    let _also = AgentProviderResponse {
        provider: resp.provider,
    };
}

#[test]
fn test_agent_skills_request_response_construct() {
    let _req = AgentSkillsRequest;
    let resp = AgentSkillsResponse { skills: vec![] };
    assert!(resp.skills.is_empty());
}

#[test]
fn test_current_state_request_response_construct() {
    let _req = CurrentStateRequest;
    let resp = CurrentStateResponse {
        state: AgentState::Idle,
    };
    assert_eq!(resp.state, AgentState::Idle);
}

#[test]
fn test_input_schema_request_response_construct() {
    let _req = InputSchemaRequest;
    let resp = InputSchemaResponse { schema: None };
    assert!(resp.schema.is_none());
}

#[test]
fn test_list_agent_ids_request_response_construct() {
    let _req = ListAgentIdsRequest;
    let resp = ListAgentIdsResponse { ids: vec![] };
    assert!(resp.ids.is_empty());
}

#[test]
fn test_message_builder_request_response_construct() {
    let _req = MessageBuilderRequest;
    let resp = MessageBuilderResponse {
        builder: Box::new(MessageBuilder::new()),
    };
    assert_eq!(resp.builder.build().role, Role::User);
}

#[test]
fn test_message_send_request_response_construct() {
    let req = MessageSendRequest {
        message: Box::new(Message::user("hi")),
    };
    assert_eq!(req.message.role, Role::User);
    let resp = MessageSendResponse { delivered: 1 };
    assert_eq!(resp.delivered, 1);
}

#[test]
fn test_output_schema_request_response_construct() {
    let _req = OutputSchemaRequest;
    let resp = OutputSchemaResponse { schema: None };
    assert!(resp.schema.is_none());
}

#[test]
fn test_parameter_documentation_builder_request_response_construct() {
    let req = ParameterDocumentationBuilderRequest {
        name: "n".to_string(),
        description: "d".to_string(),
        param_type: "string".to_string(),
        required: true,
    };
    assert_eq!(req.name, "n");
    let resp = ParameterDocumentationBuilderResponse {
        builder: Box::new(ParameterDocumentationBuilder::new("n", "d", "string", true)),
    };
    assert_eq!(resp.builder.build().name, "n");
}

#[test]
fn test_parameter_documentation_list_request_response_construct() {
    let _req = ParameterDocumentationListRequest;
    let resp = ParameterDocumentationListResponse {
        documentation: vec![],
    };
    assert!(resp.documentation.is_empty());
}

#[test]
fn test_pause_request_constructs() {
    let req = PauseRequest;
    assert_eq!(req, PauseRequest);
}

#[test]
fn test_render_content_request_response_construct() {
    let req = RenderContentRequest {
        parts: vec![ContentPart::text("hi")],
    };
    assert_eq!(req.parts.len(), 1);
    let resp = RenderContentResponse {
        content: Box::new(MessageContent::text("hi")),
    };
    assert_eq!(*resp.content, MessageContent::text("hi"));
}

#[test]
fn test_resume_request_constructs() {
    let req = ResumeRequest;
    assert_eq!(req, ResumeRequest);
}

#[test]
fn test_schema_cache_control_request_response_construct() {
    let _req = SchemaCacheControlRequest;
    let resp = SchemaCacheControlResponse {
        cache: Box::new(CacheControl::ephemeral()),
    };
    assert!(resp.cache.is_ephemeral());
}

#[test]
fn test_schema_validation_request_constructs() {
    let value = serde_json::json!({});
    let req = SchemaValidationRequest { input: &value };
    assert!(req.input.is_object());
}

#[test]
fn test_skill_description_request_response_construct() {
    let _req = SkillDescriptionRequest;
    let resp = SkillDescriptionResponse {
        description: "desc".to_string(),
    };
    assert_eq!(resp.description, "desc");
}

#[test]
fn test_skill_execution_request_response_construct() {
    
    let security = edge_security_runtime::SecurityContext::unauthenticated();
    let commands = edge_domain_command::DirectCommandBus;
    let observer = edge_domain_observer::StdObserveFactory::noop_observer_context();
    let ctx = edge_domain_handler::HandlerContext {
        security: &security,
        commands: &commands,
        observer: observer.as_ref(),
    };
    let req = SkillExecutionRequest {
        skill_name: "review",
        input: "x".to_string(),
        ctx,
    };
    assert_eq!(req.skill_name, "review");
    let resp = SkillExecutionResponse {
        output: "y".to_string(),
    };
    assert_eq!(resp.output, "y");
}

#[test]
fn test_skill_input_validation_request_constructs() {
    let req = SkillInputValidationRequest { input: "{}" };
    assert_eq!(req.input, "{}");
}

#[test]
fn test_skill_lookup_request_response_construct() {
    let req = SkillLookupRequest { name: "review" };
    assert_eq!(req.name, "review");
    let skills = NoopAgent.skills(AgentSkillsRequest).unwrap().skills;
    assert!(skills.is_empty());
}

#[test]
fn test_skill_metadata_builder_request_response_construct() {
    let _req = SkillMetadataBuilderRequest;
    let resp = SkillMetadataBuilderResponse {
        builder: Box::new(SkillMetadataBuilder::new()),
    };
    assert_eq!(resp.builder.build().name, "");
}

#[test]
fn test_skill_metadata_lookup_request_response_construct() {
    let _req = SkillMetadataLookupRequest;
    let resp = SkillMetadataLookupResponse {
        metadata: Box::new(SkillMetadata::default()),
    };
    assert_eq!(resp.metadata.name, "");
}

#[test]
fn test_skill_name_request_response_construct() {
    let _req = SkillNameRequest;
    let resp = SkillNameResponse {
        name: "review".to_string(),
    };
    assert_eq!(resp.name, "review");
}

#[test]
fn test_skill_name_validation_request_constructs() {
    let req = SkillNameValidationRequest {
        skill_name: "review",
    };
    assert_eq!(req.skill_name, "review");
}

#[test]
fn test_skill_parameters_request_response_construct() {
    let _req = SkillParametersRequest;
    let resp = SkillParametersResponse { parameters: vec![] };
    assert!(resp.parameters.is_empty());
}

#[test]
fn test_state_check_request_response_construct() {
    let req = StateCheckRequest {
        state: AgentState::Idle,
    };
    assert_eq!(req.state, AgentState::Idle);
    let resp = StateCheckResponse { matches: true };
    assert!(resp.matches);
}

#[test]
fn test_supported_role_request_response_construct() {
    let _req = SupportedRoleRequest;
    let resp = SupportedRoleResponse { role: Role::User };
    assert_eq!(resp.role, Role::User);
}

#[test]
fn test_tool_call_validation_request_constructs() {
    let call = ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: "{}".to_string(),
    };
    let req = ToolCallValidationRequest { call: &call };
    assert_eq!(req.call.name, "search");
}

#[test]
fn test_tool_choice_preference_request_response_construct() {
    let _req = ToolChoicePreferenceRequest;
    let resp = ToolChoicePreferenceResponse {
        choice: ToolChoice::Auto,
    };
    assert_eq!(resp.choice, ToolChoice::Auto);
}

#[test]
fn test_transition_request_constructs() {
    let req = TransitionRequest {
        target: AgentState::Running,
    };
    assert_eq!(req.target, AgentState::Running);
}
