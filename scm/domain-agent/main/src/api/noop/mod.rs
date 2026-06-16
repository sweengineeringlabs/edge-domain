//! No-op marker interface module.
//!
//! Markers for no-op implementations that satisfy core_api_module_correspondence rule.

mod noop_agent;
mod noop_agent_manager;
mod noop_agent_registry;
mod noop_skill;
mod noop_validator;

pub use noop_agent::NoopAgent;
pub use noop_agent_manager::NoopAgentManager;
pub use noop_agent_registry::NoopAgentRegistry;
pub use noop_skill::NoopSkill;
pub use noop_validator::NoopValidator;
