//! Trait implementations for agent value types.

pub(crate) mod agent_metadata;
pub(crate) mod agent_metadata_builder;
pub(crate) mod agent_state;
pub(crate) mod cache_control;
pub(crate) mod content_part;
pub(crate) mod default_agent;
pub(crate) mod input_output_schema;
pub(crate) mod message;
pub(crate) mod message_builder;
pub(crate) mod message_content;
pub(crate) mod parameter_documentation;
pub(crate) mod parameter_documentation_builder;
pub(crate) mod skill_metadata;
pub(crate) mod skill_metadata_builder;
pub(crate) mod validation_error;
pub(crate) use default_agent::DefaultAgentHandler;
