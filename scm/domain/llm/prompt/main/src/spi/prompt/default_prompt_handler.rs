//! `DefaultPromptHandler` — self-contained `Handler` impl for the prompt primitive (ADR-024).
//!
//! This file is intentionally self-contained: it does NOT import from `crate::core`.
//! It mirrors `core/prompt/default_prompt.rs` so that `saf/` can compose it
//! without crossing the `core/` boundary (SEA §7 / boundary_peer_isolation).

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{Handler, HandlerContext, HandlerError};

use crate::api::{Prompt, RenderContext};

/// Stable handler id under which this handler registers for dispatch.
const PROMPT_HANDLER_ID: &str = "prompt.render";
/// Route pattern this handler matches in the dispatch table.
const PROMPT_HANDLER_PATTERN: &str = "prompt/render";

pub(crate) struct DefaultPromptHandler {
    pub(crate) prompt: Arc<dyn Prompt>,
}

#[async_trait]
impl Handler for DefaultPromptHandler {
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
    use crate::api::{PromptMetadata, StaticPrompt, Variable, VariableType};
    use edge_domain_command::{CommandBusBootstrap, StdCommandBusFactory};
    use edge_domain_security::SecurityContext;
    use futures::executor::block_on;

    fn handler() -> DefaultPromptHandler {
        let var = Variable::new("name".to_string(), VariableType::String);
        let metadata = PromptMetadata::new(
            "greet".to_string(),
            "Greeting".to_string(),
            "1".to_string(),
            vec![var],
        );
        DefaultPromptHandler {
            prompt: Arc::new(StaticPrompt::new("Hello {{name}}".to_string(), metadata)),
        }
    }

    #[test]
    fn test_handler_execute_renders_template_happy() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &commands);
        let context = RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        let out = block_on(Handler::execute(&handler(), context, ctx)).expect("handler ok");
        assert_eq!(out, "Hello Ada");
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(Handler::id(&handler()), PROMPT_HANDLER_ID);
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(Handler::pattern(&handler()), PROMPT_HANDLER_PATTERN);
    }

    #[test]
    fn test_handler_execute_missing_variable_error() {
        let security = SecurityContext::unauthenticated();
        let commands = StdCommandBusFactory::direct();
        let ctx = HandlerContext::new(&security, &commands);
        let empty_ctx = RenderContext::new();
        let result = block_on(Handler::execute(&handler(), empty_ctx, ctx));
        assert!(result.is_err());
    }
}
