//! No-op [`AgentManager`] implementation and agent handler factory.

use std::sync::Arc;

use crate::api::{
    AgentCreationRequest, AgentCreationResponse, AgentHandlerRequest, AgentHandlerResponse,
    AgentLoadRequest, AgentLoadResponse, AgentLookupRequest, AgentLookupResponse,
    ListAgentIdsRequest, ListAgentIdsResponse,
};
use crate::api::{AgentError, AgentManager, NoopAgentManager};
use crate::core::noop::DefaultAgent;
use crate::core::types::DefaultAgentHandler;

#[async_trait::async_trait]
impl AgentManager for NoopAgentManager {
    async fn load_agent(
        &self,
        _req: AgentLoadRequest<'_>,
    ) -> Result<AgentLoadResponse, AgentError> {
        Err(AgentError::InvalidSpec("No-op manager".to_string()))
    }

    fn agent(&self, req: AgentLookupRequest<'_>) -> Result<AgentLookupResponse, AgentError> {
        Err(AgentError::NotFound(req.id.to_string()))
    }

    fn list_agent_ids(
        &self,
        _req: ListAgentIdsRequest,
    ) -> Result<ListAgentIdsResponse, AgentError> {
        Ok(ListAgentIdsResponse { ids: vec![] })
    }

    fn agent_handler(
        &self,
        req: AgentHandlerRequest<'_>,
    ) -> Result<AgentHandlerResponse, AgentError> {
        Ok(AgentHandlerResponse {
            handler: Box::new(DefaultAgentHandler {
                skill: req.skill.to_string(),
            }),
        })
    }

    fn default_agent(
        &self,
        req: AgentCreationRequest<'_>,
    ) -> Result<AgentCreationResponse, AgentError> {
        Ok(AgentCreationResponse {
            agent: Arc::new(DefaultAgent::new(
                req.id,
                req.name,
                req.description,
                req.provider,
                req.skills,
            )),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_handler::{ExecutionRequest, Handler, HandlerContext};
    use edge_domain_observer::StdObserveFactory;
    use edge_domain_security::{SecurityBootstrap, SecurityServices};
    use futures::executor::block_on;

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_happy_routes_skill_to_input() {
        let h = NoopAgentManager
            .agent_handler(AgentHandlerRequest { skill: "review" })
            .unwrap()
            .handler;
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let out = block_on(Handler::execute(
            &*h,
            ExecutionRequest {
                req: "code".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("ok");
        assert_eq!(out, "review:code");
    }

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_error_rejects_empty_input() {
        let h = NoopAgentManager
            .agent_handler(AgentHandlerRequest { skill: "review" })
            .unwrap()
            .handler;
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        assert!(block_on(Handler::execute(
            &*h,
            ExecutionRequest {
                req: String::new(),
                ctx: &ctx,
            }
        ))
        .is_err());
    }

    /// @covers: agent_handler
    #[test]
    fn test_agent_handler_edge_empty_skill_preserved() {
        let h = NoopAgentManager
            .agent_handler(AgentHandlerRequest { skill: "" })
            .unwrap()
            .handler;
        let security = SecurityServices::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let out = block_on(Handler::execute(
            &*h,
            ExecutionRequest {
                req: "x".to_string(),
                ctx: &ctx,
            },
        ))
        .expect("ok");
        assert_eq!(out, ":x");
    }

    #[test]
    fn test_noop_agent_manager_happy_list_agent_ids_returns_empty() {
        let ids = NoopAgentManager
            .list_agent_ids(ListAgentIdsRequest)
            .map(|r| r.ids)
            .unwrap_or_default();
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_noop_agent_manager_error_agent_returns_not_found() {
        let result = NoopAgentManager.agent(AgentLookupRequest { id: "any" });
        assert!(matches!(result, Err(AgentError::NotFound(_))));
    }

    #[test]
    fn test_noop_agent_manager_error_load_agent_returns_invalid_spec() {
        let result = futures::executor::block_on(
            NoopAgentManager.load_agent(AgentLoadRequest { spec: "spec" }),
        );
        assert!(matches!(result, Err(AgentError::InvalidSpec(_))));
    }
}
