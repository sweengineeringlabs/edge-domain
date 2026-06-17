//! No-op marker types.
//!
//! These zero-sized markers are the api-layer counterparts of the no-op trait
//! implementations in `core/noop/`. They satisfy the
//! `core_api_module_correspondence` rule (every `core/` submodule has a real
//! api/ counterpart) and give `core/` concrete types to implement traits for.

mod noop_agent;
mod noop_agent_lifecycle;
mod noop_agent_manager;
mod noop_agent_registry;
mod noop_schema_validator;
mod noop_skill;
mod noop_validator;

pub use noop_agent::NoopAgent;
pub use noop_agent_lifecycle::NoopAgentLifecycle;
pub use noop_agent_manager::NoopAgentManager;
pub use noop_agent_registry::NoopAgentRegistry;
pub use noop_schema_validator::NoopSchemaValidator;
pub use noop_skill::NoopSkill;
pub use noop_validator::NoopValidator;
