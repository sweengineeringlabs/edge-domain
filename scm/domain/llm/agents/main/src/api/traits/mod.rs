//! Agent domain trait contracts.
pub mod agent;
pub mod agent_manager;
pub mod agent_registry;
pub mod conversation_loop;
pub mod parameter;
pub mod skill;
pub mod validator;

pub use agent::Agent;
pub use agent_manager::AgentManager;
pub use agent_registry::AgentRegistry;
pub use conversation_loop::ConversationLoop;
pub use parameter::Parameter;
pub use skill::Skill;
pub use validator::Validator;

// LLM agent traits (merged from edge_llm_agent)
pub mod agent_lifecycle;
pub mod schema_validator;

pub use agent_lifecycle::AgentLifecycle;
pub use schema_validator::SchemaValidator;
