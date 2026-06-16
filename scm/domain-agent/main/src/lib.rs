//! # edge-domain-agent
//!
//! Agent domain primitives: `Agent`, `Skill`, `AgentManager`, `AgentRegistry`.
//!
//! Agents are first-class domain concepts. This crate defines the contracts;
//! concrete implementations live in plugins (e.g., `edge-plugin-llmboot`).

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use saf::{
    Agent, AgentError, AgentManager, AgentMetadata, AgentRegistry, Parameter, Skill, SkillMetadata,
    AGENT_ERROR_SVC, AGENT_MANAGER_SVC, AGENT_METADATA_SVC, AGENT_REGISTRY_SVC, AGENT_SVC,
    PARAMETER_SVC, SKILL_METADATA_SVC, SKILL_SVC,
};
