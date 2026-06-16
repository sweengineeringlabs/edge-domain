mod agent_svc;
mod agent_manager_svc;
mod agent_registry_svc;

pub use agent_svc::AGENT_SVC;
pub use agent_manager_svc::AGENT_MANAGER_SVC;
pub use agent_registry_svc::AGENT_REGISTRY_SVC;

pub use crate::api::{Agent, AgentError, AgentManager, AgentMetadata, AgentRegistry, Skill, SkillMetadata};
