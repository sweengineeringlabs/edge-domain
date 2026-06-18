//! `Handler` impl for `PromptEndpoint` (ADR-024).

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{PromptEndpoint, RenderContext};

/// Stable handler id under which the endpoint registers for dispatch.
const PROMPT_HANDLER_ID: &str = "prompt.render";
/// Route pattern this endpoint matches in the dispatch table.
const PROMPT_HANDLER_PATTERN: &str = "prompt/render";

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::api::{PromptMetadata, StaticPrompt, Variable, VariableType};
    use edge_domain_command::{CommandBusFactory, StdCommandBusFactory};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn endpoint() -> PromptEndpoint {
        let var = Variable::new("name".to_string(), VariableType::String);
        let metadata = PromptMetadata::new(
            "greet".to_string(),
            "Greeting".to_string(),
            "1".to_string(),
            vec![var],
        );
        PromptEndpoint::new(Arc::new(StaticPrompt::new(
            "Hello {{name}}".to_string(),
            metadata,
        )))
    }

    #[test]
    fn test_handler_execute_renders_template_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        let out = block_on(Handler::execute(&endpoint(), context, ctx)).expect("handler ok");
        assert_eq!(out, "Hello Ada");
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&endpoint()), PROMPT_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&endpoint()), PROMPT_HANDLER_PATTERN);
    }

    #[test]
    fn test_handler_execute_missing_variable_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext { security: &security, commands: &commands };
        let empty_ctx = RenderContext::new();
        let result = block_on(Handler::execute(&endpoint(), empty_ctx, ctx));
        assert!(result.is_err());
    }
}
