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

// Legacy domain-agent exports (Agent, Skill, etc.)
pub use saf::{
    Agent, AgentError, AgentManager, AgentMetadata, AgentMetadataBuilder, AgentRegistry, Parameter,
    Skill, SkillMetadata, SkillMetadataBuilder, Validator, AGENT_ERROR_SVC, AGENT_MANAGER_SVC,
    AGENT_METADATA_BUILDER_SVC, AGENT_METADATA_SVC, AGENT_REGISTRY_SVC, AGENT_SVC, PARAMETER_SVC,
    SKILL_METADATA_BUILDER_SVC, SKILL_METADATA_SVC, SKILL_SVC, VALIDATOR_SVC,
};

// LLM agent types and traits
pub use api::{
    AgentState, AgentLifecycleError, Message, MessageContent, ContentPart, Role, ToolChoice,
    ToolCall, CacheControl, ParameterDocumentation, InputOutputSchema, ValidationError,
    AgentLifecycle, SchemaValidator,
};
