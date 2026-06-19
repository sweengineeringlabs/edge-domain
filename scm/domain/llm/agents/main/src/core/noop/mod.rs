//! No-op trait implementations for contract testing.
//!
//! Each file implements one api/ trait for the matching marker type in
//! `crate::api::noop`. Real implementations live in plugins, not here.

pub(crate) mod default_agent;
pub(crate) mod noop_agent;
pub(crate) mod noop_agent_lifecycle;
pub(crate) mod noop_agent_manager;
pub(crate) mod noop_agent_registry;
pub(crate) mod noop_schema_validator;
pub(crate) mod noop_skill;
pub(crate) mod noop_validator;

pub(crate) use default_agent::DefaultAgent;
