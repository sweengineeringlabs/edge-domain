//! `Handler` + `Service` impls for `PromptEndpoint` (ADR-037 connection).

use async_trait::async_trait;
use futures::future::BoxFuture;

use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
use edge_domain_handler::{Handler, HandlerContext, HandlerError};
use edge_domain_security::SecurityContext;
use edge_domain_service::{Service, ServiceError};

use crate::api::Prompt;
use crate::api::{PromptEndpoint, RenderContext};

/// Stable handler id under which the endpoint registers for dispatch.
const PROMPT_HANDLER_ID: &str = "prompt.render";
/// Route pattern this endpoint matches in the dispatch table.
const PROMPT_HANDLER_PATTERN: &str = "prompt/render";
/// Stable service name under which consumers resolve this endpoint.
const PROMPT_SERVICE_NAME: &str = "prompt";

#[async_trait]
impl Handler for PromptEndpoint {
    type Request = RenderContext;
    type Response = String;

    fn id(&self) -> &str {
        PROMPT_HANDLER_ID
    }

    fn pattern(&self) -> &str {
        PROMPT_HANDLER_PATTERN
    }

    async fn execute(
        &self,
        context: RenderContext,
        _ctx: HandlerContext<'_>,
    ) -> Result<String, HandlerError> {
        self.prompt
            .render(&context)
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.message()))
    }
}

impl Service for PromptEndpoint {
    type Request = RenderContext;
    type Response = String;

    fn name(&self) -> &str {
        PROMPT_SERVICE_NAME
    }

    fn execute(&self, context: RenderContext) -> BoxFuture<'_, Result<String, ServiceError>> {
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
            Handler::execute(self, context, ctx)
                .await
                .map_err(|e| ServiceError::Internal(e.to_string()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{PromptMetadata, Variable, VariableType};
    use futures::executor::block_on;

    fn endpoint() -> PromptEndpoint {
        let var = Variable::new("name".to_string(), VariableType::String);
        let metadata = PromptMetadata::new(
            "greet".to_string(),
            "Greeting".to_string(),
            "1".to_string(),
            vec![var],
        );
        PromptEndpoint::new(crate::api::StaticPrompt::new(
            "Hello {{name}}".to_string(),
            metadata,
        ))
    }

    #[test]
    fn test_service_delegates_to_handler() {
        let ctx = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        let out = block_on(Service::execute(&endpoint(), ctx)).expect("service ok");
        assert_eq!(out, "Hello Ada");
    }

    #[test]
    fn test_handler_id_is_stable() {
        assert_eq!(Handler::id(&endpoint()), PROMPT_HANDLER_ID);
    }

    #[test]
    fn test_service_surfaces_render_error() {
        let ctx = RenderContext::new();
        assert!(block_on(Service::execute(&endpoint(), ctx)).is_err());
    }
}
