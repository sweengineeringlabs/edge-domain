//! # edge_llm_agent
//!
//! LLM Agent domain primitive: lifecycle state machine, messaging, tool governance.
//!
//! Consolidates agent orchestration (Agent, Skill, AgentManager, AgentRegistry) with
//! LLM-specific features (AgentLifecycle, Message, ToolChoice, SchemaValidator).

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

// Trait + service exports via the SAF surface (Agent, Skill, lifecycle, etc.).
pub use saf::{
    Agent, AgentLifecycle, AgentManager, AgentRegistry, ConversationLoop, SchemaValidator, Skill,
    Validator, AGENT_LIFECYCLE_SVC, AGENT_MANAGER_SVC, AGENT_METADATA_BUILDER_SVC,
    AGENT_REGISTRY_SVC, AGENT_SVC, CONVERSATION_LOOP_SVC, DEFAULT_AGENT_SVC, SCHEMA_VALIDATOR_SVC,
    SKILL_METADATA_BUILDER_SVC, SKILL_SVC, VALIDATOR_SVC,
};

// Re-export SAF factory markers
pub use saf::{
    AGENT_LIFECYCLE_SVC_FACTORY, AGENT_MANAGER_SVC_FACTORY, AGENT_REGISTRY_SVC_FACTORY,
    AGENT_SVC_FACTORY, CONVERSATION_LOOP_SVC_FACTORY, SCHEMA_VALIDATOR_SVC_FACTORY,
    SKILL_SVC_FACTORY, VALIDATOR_SVC_FACTORY,
};

// Re-export API value types for integration tests and client libraries
pub use api::{
    AgentError, AgentLifecycleError, AgentMetadata, AgentMetadataBuilder, AgentState,
    BoundedConversationLoop, CacheControl, ContentPart, InputOutputSchema, Message, MessageBuilder,
    MessageContent, NoopAgent, NoopAgentLifecycle, NoopAgentManager, NoopAgentRegistry,
    NoopSchemaValidator, NoopSkill, NoopValidator, OwnedHandlerContext, Parameter,
    ParameterDocumentation, ParameterDocumentationBuilder, Role, SkillMetadata,
    SkillMetadataBuilder, ToolCall, ToolChoice, ValidationError,
};

// Re-export Request/Response envelope types used by trait method signatures
pub use api::{
    AbortRequest, AgentCreationRequest, AgentCreationResponse, AgentDescriptionRequest,
    AgentDescriptionResponse, AgentHandlerRequest, AgentHandlerResponse, AgentIdRequest,
    AgentIdResponse, AgentIdValidationRequest, AgentLoadRequest, AgentLoadResponse,
    AgentLookupRequest, AgentLookupResponse, AgentMetadataBuilderRequest,
    AgentMetadataBuilderResponse, AgentMetadataLookupRequest, AgentMetadataLookupResponse,
    AgentNameRequest, AgentNameResponse, AgentProviderRequest, AgentProviderResponse,
    AgentSkillsRequest, AgentSkillsResponse, ConversationLoopRequest, ConversationLoopResponse,
    ConversationRunRequest, ConversationRunResponse, CurrentStateRequest, CurrentStateResponse,
    InputSchemaRequest, InputSchemaResponse, ListAgentIdsRequest, ListAgentIdsResponse,
    MessageBuilderRequest, MessageBuilderResponse, MessageSendRequest, MessageSendResponse,
    OutputSchemaRequest, OutputSchemaResponse, ParameterDocumentationBuilderRequest,
    ParameterDocumentationBuilderResponse, ParameterDocumentationListRequest,
    ParameterDocumentationListResponse, PauseRequest, RenderContentRequest, RenderContentResponse,
    ResumeRequest, SchemaCacheControlRequest, SchemaCacheControlResponse, SchemaValidationRequest,
    SkillDescriptionRequest, SkillDescriptionResponse, SkillExecutionRequest,
    SkillExecutionResponse, SkillInputValidationRequest, SkillLookupRequest, SkillLookupResponse,
    SkillMetadataBuilderRequest, SkillMetadataBuilderResponse, SkillMetadataLookupRequest,
    SkillMetadataLookupResponse, SkillNameRequest, SkillNameResponse, SkillNameValidationRequest,
    SkillParametersRequest, SkillParametersResponse, StateCheckRequest, StateCheckResponse,
    SupportedRoleRequest, SupportedRoleResponse, ToolCallValidationRequest,
    ToolChoicePreferenceRequest, ToolChoicePreferenceResponse, TransitionRequest,
};
