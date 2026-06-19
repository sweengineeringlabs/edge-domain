//! No-op [`AgentManager`] implementation and agent handler factory.

use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{Agent, AgentEndpoint, AgentError, AgentManager, NoopAgentManager};
use crate::core::types::DefaultAgentHandler;

impl AgentEndpoint for NoopAgentManager {
    fn agent_handler(
        skill: impl Into<String>,
    ) -> impl Handler<Request = String, Response = String> {
        DefaultAgentHandler { skill: skill.into() }
    }
}

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_handler::{Handler, HandlerContext};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_happy_routes_skill_to_input() {
        let h = <NoopAgentManager as AgentEndpoint>::agent_handler("review");
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &commands);
        let out = block_on(Handler::execute(&h, "code".to_string(), ctx)).expect("ok");
        assert_eq!(out, "review:code");
    }

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_error_rejects_empty_input() {
        let h = <NoopAgentManager as AgentEndpoint>::agent_handler("review");
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &commands);
        assert!(block_on(Handler::execute(&h, String::new(), ctx)).is_err());
    }

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_edge_empty_skill_preserved() {
        let h = <NoopAgentManager as AgentEndpoint>::agent_handler("");
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &commands);
        let out = block_on(Handler::execute(&h, "x".to_string(), ctx)).expect("ok");
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
