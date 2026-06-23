//! # edge_llm_agent
//!
//! LLM Agent domain primitive: lifecycle state machine, messaging, tool governance.
//!
//! Consolidates agent orchestration (Agent, Skill, AgentManager, AgentRegistry) with
//! LLM-specific features (AgentLifecycle, Message, ToolChoice, SchemaValidator).

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;
mod spi;

// Trait + service exports via the SAF surface (Agent, Skill, lifecycle, etc.).
pub use saf::{
    Agent, AgentLifecycle, AgentManager,
    AgentRegistry, SchemaValidator, Skill,
    Validator, AGENT_LIFECYCLE_SVC,
    AGENT_MANAGER_SVC, AGENT_METADATA_BUILDER_SVC, AGENT_REGISTRY_SVC,
    AGENT_SVC, DEFAULT_AGENT_SVC,
    SCHEMA_VALIDATOR_SVC,
    SKILL_METADATA_BUILDER_SVC, SKILL_SVC, VALIDATOR_SVC,
};

// Re-export API value types for integration tests and client libraries
pub use api::{
    AgentError,
    AgentLifecycleError, AgentMetadata, AgentMetadataBuilder, AgentState,
    CacheControl, ContentPart,
    InputOutputSchema,
    Message, MessageBuilder, MessageContent,
    NoopAgent, NoopAgentLifecycle, NoopAgentManager, NoopAgentRegistry,
    NoopSchemaValidator, NoopSkill, NoopValidator,
    Parameter, ParameterDocumentation, ParameterDocumentationBuilder,
    Role, SkillMetadata, SkillMetadataBuilder,
    ToolCall, ToolChoice, ValidationError,
};
