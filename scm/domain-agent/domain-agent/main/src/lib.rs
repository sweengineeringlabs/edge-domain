//! # edge-domain-agent
//!
//! Agent domain primitives: `Agent`, `Skill`, `AgentController`, `AgentRegistry`.
//!
//! Agents are first-class domain concepts. This crate defines the contracts;
//! concrete implementations live in plugins (e.g., `edge-plugin-llmboot`).

#![deny(unsafe_code)]
#![warn(missing_docs)]

mod api;
mod core;
mod saf;

pub use api::{Agent, AgentController, AgentError, AgentMetadata, AgentRegistry, Skill, SkillMetadata};
