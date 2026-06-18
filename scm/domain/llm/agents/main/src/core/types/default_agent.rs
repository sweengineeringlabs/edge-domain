//! `Handler` impl for `AgentEndpoint` (ADR-024).

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::AgentEndpoint;

/// Stable handler id under which the endpoint registers for dispatch.
const AGENT_HANDLER_ID: &str = "agent.execute_skill";
/// Route pattern this endpoint matches in the dispatch table.
const AGENT_HANDLER_PATTERN: &str = "agent/execute_skill";

#[async_trait::async_trait]
impl Handler for AgentEndpoint {
    type Request = String;
    type Response = String;

    fn id(&self) -> &str {
        AGENT_HANDLER_ID
    }

    fn pattern(&self) -> &str {
        AGENT_HANDLER_PATTERN
    }

    async fn execute(
        &self,
        input: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        if input.is_empty() {
            return Err(HandlerError::ExecutionFailed(
                "agent skill input must not be empty".to_string(),
            ));
        }
        Ok(format!("{}:{}", self.skill(), input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn endpoint() -> AgentEndpoint {
        AgentEndpoint::new("code_review")
    }

    #[test]
    fn test_handler_execute_returns_skill_colon_input_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let out = block_on(Handler::execute(&endpoint(), "diff".to_string(), ctx))
            .expect("handler ok");
        assert_eq!(out, "code_review:diff");
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&endpoint()), AGENT_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&endpoint()), AGENT_HANDLER_PATTERN);
    }

    #[test]
    fn test_handler_execute_empty_input_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let result = block_on(Handler::execute(&endpoint(), String::new(), ctx));
        assert!(result.is_err());
    }
}
