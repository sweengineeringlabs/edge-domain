//! No-op [`AgentManager`] implementation and agent handler factory.

use std::sync::Arc;

use edge_domain_handler::Handler;
use edge_llm_provider::Provider;

use crate::api::{Agent, AgentError, AgentManager, NoopAgentManager, Skill};
use crate::core::noop::DefaultAgent;
use crate::core::types::DefaultAgentHandler;

#[async_trait::async_trait]
impl AgentManager for NoopAgentManager {
    async fn load_agent(&self, _spec: &str) -> Result<Arc<dyn Agent>, AgentError> {
        Err(AgentError::InvalidSpec("No-op manager".to_string()))
    }

    fn agent(&self, id: &str) -> Result<Arc<dyn Agent>, AgentError> {
        Err(AgentError::NotFound(id.to_string()))
    }

    fn list_agent_ids(&self) -> Result<Vec<String>, AgentError> {
        Ok(vec![])
    }

    fn agent_handler(&self, skill: &str) -> Box<dyn Handler<Request = String, Response = String>> {
        Box::new(DefaultAgentHandler {
            skill: skill.to_string(),
        })
    }

    fn default_agent(
        &self,
        id: &str,
        name: &str,
        description: &str,
        provider: Arc<dyn Provider>,
        skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
    ) -> Arc<dyn Agent> {
        Arc::new(DefaultAgent::new(id, name, description, provider, skills))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_observe::StdObserveFactory;
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_happy_routes_skill_to_input() {
        let h = NoopAgentManager.agent_handler("review");
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let out = block_on(Handler::execute(&*h, "code".to_string(), ctx)).expect("ok");
        assert_eq!(out, "review:code");
    }

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_error_rejects_empty_input() {
        let h = NoopAgentManager.agent_handler("review");
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        assert!(block_on(Handler::execute(&*h, String::new(), ctx)).is_err());
    }

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_edge_empty_skill_preserved() {
        let h = NoopAgentManager.agent_handler("");
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observe_context();
        let ctx = HandlerContext::new(&security, &commands, observer.as_ref());
        let out = block_on(Handler::execute(&*h, "x".to_string(), ctx)).expect("ok");
        assert_eq!(out, ":x");
    }

    #[test]
    fn test_noop_agent_manager_happy_list_agent_ids_returns_empty() {
        let ids = NoopAgentManager.list_agent_ids().unwrap_or_default();
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_noop_agent_manager_error_agent_returns_not_found() {
        let result = NoopAgentManager.agent("any");
        assert!(matches!(result, Err(AgentError::NotFound(_))));
    }

    #[test]
    fn test_noop_agent_manager_error_load_agent_returns_invalid_spec() {
        let result = futures::executor::block_on(NoopAgentManager.load_agent("spec"));
        assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
    }
}
