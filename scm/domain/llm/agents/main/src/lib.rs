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

// Trait + service exports via the SAF surface (Agent, Skill, lifecycle, etc.).
pub use saf::{
    Agent, AgentError, AgentLifecycle, AgentManager, AgentMetadata, AgentMetadataBuilder,
    AgentRegistry, NoopAgent, NoopAgentLifecycle, NoopAgentManager, NoopAgentRegistry,
    NoopSchemaValidator, NoopSkill, NoopValidator, Parameter, SchemaValidator, Skill,
    SkillMetadata, SkillMetadataBuilder, Validator, AGENT_ERROR_SVC, AGENT_LIFECYCLE_SVC,
    AGENT_MANAGER_SVC, AGENT_METADATA_BUILDER_SVC, AGENT_METADATA_SVC, AGENT_REGISTRY_SVC,
    AGENT_SVC, PARAMETER_SVC, SCHEMA_VALIDATOR_SVC, SKILL_METADATA_BUILDER_SVC, SKILL_METADATA_SVC,
    SKILL_SVC, VALIDATOR_SVC,
};

// LLM agent value types and builders.
pub use api::{
    AgentLifecycleError, AgentState, CacheControl, ContentPart, InputOutputSchema, Message,
    MessageBuilder, MessageContent, ParameterDocumentation, ParameterDocumentationBuilder, Role,
    ToolCall, ToolChoice, ValidationError,
};
