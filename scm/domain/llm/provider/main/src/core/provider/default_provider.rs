//! `Handler` + `Service` impls for `DefaultProvider` (ADR-037 connection).

use async_trait::async_trait;
use futures::future::BoxFuture;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceError};

use crate::api::ExecutionModel;
use crate::api::{ExecutionStepResult, DefaultProvider};

/// Stable handler id under which the endpoint registers for dispatch.
const PROVIDER_HANDLER_ID: &str = "provider.execute_step";
/// Route pattern this endpoint matches in the dispatch table.
const PROVIDER_HANDLER_PATTERN: &str = "provider/execute_step";
/// Stable service name under which consumers resolve this endpoint.
const PROVIDER_SERVICE_NAME: &str = "provider";

#[async_trait]
impl Handler for DefaultProvider {
    type Request = String;
    type Response = ExecutionStepResult;

    fn id(&self) -> &str {
        PROVIDER_HANDLER_ID
    }

    fn pattern(&self) -> &str {
        PROVIDER_HANDLER_PATTERN
    }

    async fn execute(
        &self,
        goal: String,
        _ctx: HandlerContext<'_>,
    ) -> Result<ExecutionStepResult, HandlerError> {
        self.model
            .execute_step("", &goal, "", Vec::new())
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.message()))
    }
}

impl Service for DefaultProvider {
    type Request = String;
    type Response = ExecutionStepResult;

    fn name(&self) -> &str {
        PROVIDER_SERVICE_NAME
    }

    fn execute(&self, goal: String) -> BoxFuture<'_, Result<ExecutionStepResult, ServiceError>> {
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
            Handler::execute(self, goal, ctx)
                .await
                .map_err(|e| ServiceError::Internal(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{EchoExecutionModel, ExecutionConfig, ExecutionMode};
    use futures::executor::block_on;

    fn endpoint() -> DefaultProvider {
        let config = ExecutionConfig::new(4096, 30_000, true, false, ExecutionMode::Async);
        DefaultProvider::new(EchoExecutionModel::new(config))
    }

    #[test]
    fn test_service_delegates_to_handler() {
        let out = block_on(Service::execute(&endpoint(), "ship".to_string())).expect("service ok");
        assert!(out.reasoning.contains("ship"));
    }

    #[test]
    fn test_handler_id_is_stable() {
        assert_eq!(Handler::id(&endpoint()), PROVIDER_HANDLER_ID);
    }
}
