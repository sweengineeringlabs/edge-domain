//! `Handler` + `Service` impls for `AgentEndpoint` (ADR-037 connection).
//!
//! The `Handler` face runs the skill-execution core under a request context;
//! the `Service` face funnels into that same `Handler` so the typed, named
//! consumption path cannot bypass the dispatch pipeline (Service → Dispatch →
//! Handler → core).

use futures::future::BoxFuture;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceError};

use crate::api::AgentEndpoint;

/// Stable handler id under which the endpoint registers for dispatch.
const AGENT_HANDLER_ID: &str = "agent.execute_skill";
/// Route pattern this endpoint matches in the dispatch table.
const AGENT_HANDLER_PATTERN: &str = "agent/execute_skill";
/// Stable service name under which consumers resolve this endpoint.
const AGENT_SERVICE_NAME: &str = "agent";

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

impl Service for AgentEndpoint {
    type Request = String;
    type Response = String;

    fn name(&self) -> &str {
        AGENT_SERVICE_NAME
    }

    fn execute(&self, input: String) -> BoxFuture<'_, Result<String, ServiceError>> {
        // ADR-037: Service → Dispatch → Handler → core. This in-crate reference
        // builds a default request context; in production the dispatch runtime
        // supplies the per-request `HandlerContext` (security + command bus).
        Box::pin(async move {
            let security = SecurityContext::unauthenticated();
            let commands = StdCommandBusFactory::direct();
            let ctx = HandlerContext {
                security: &security,
                commands: &commands,
            };
            Handler::execute(self, input, ctx)
                .await
                .map_err(|e| ServiceError::Internal(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;

    fn endpoint() -> AgentEndpoint {
        AgentEndpoint::new("code_review")
    }

    #[test]
    fn test_service_execute_delegates_to_handler_happy() {
        match block_on(Service::execute(&endpoint(), "diff".to_string())) {
            Ok(out) => assert_eq!(out, "code_review:diff"),
            Err(e) => panic!("expected ok, got error: {e}"),
        }
    }

    #[test]
    fn test_service_execute_propagates_handler_error() {
        let err = block_on(Service::execute(&endpoint(), String::new()));
        assert!(matches!(err, Err(ServiceError::Internal(_))));
    }

    #[test]
    fn test_handler_id_and_service_name_distinct() {
        let ep = endpoint();
        assert_eq!(Handler::id(&ep), AGENT_HANDLER_ID);
        assert_eq!(Service::name(&ep), AGENT_SERVICE_NAME);
    }
}
