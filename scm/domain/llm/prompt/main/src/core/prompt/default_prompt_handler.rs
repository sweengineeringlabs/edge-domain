//! `DefaultPromptHandler` — `Handler` impl for the prompt primitive (ADR-024).

use std::sync::Arc;

use async_trait::async_trait;

use edge_domain_handler::{
    ExecutionRequest, Handler, HandlerError, IdRequest, IdResponse, PatternRequest, PatternResponse,
};
use edge_domain_observer::{SpanFinishRequest, SpanStartRequest, TracerRequest};

use crate::api::{Prompt, RenderContext, RenderRequest};

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

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> {
        Ok(IdResponse {
            id: PROMPT_HANDLER_ID.to_string(),
        })
    }

    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> {
        Ok(PatternResponse {
            pattern: PROMPT_HANDLER_PATTERN.to_string(),
        })
    }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, RenderContext>,
    ) -> Result<String, HandlerError> {
        let span = req
            .ctx
            .observer
            .tracer(TracerRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .tracer
            .start_span(SpanStartRequest {
                handler_id: "prompt".to_string(),
                operation: "render".to_string(),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .span;
        let result = self
            .prompt
            .render(RenderRequest { context: &req.req })
            .await
            .map(|resp| resp.rendered)
            .map_err(|e| HandlerError::ExecutionFailed(e.message()));
        span.finish(SpanFinishRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{PromptMetadata, StaticPrompt, Variable, VariableKind};
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::HandlerContext;
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;

    fn handler() -> DefaultPromptHandler {
        let var = Variable::new("name".to_string(), VariableKind::String);
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
        let security: SecurityContext = SecurityContext::unauthenticated();
        let commands = DirectCommandBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let context =
            RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        let out = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: context,
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
        assert_eq!(out, "Hello Ada");
    }

    #[test]
    fn test_handler_execute_opens_and_finishes_span_happy() {
        let security: SecurityContext = SecurityContext::unauthenticated();
        let commands = DirectCommandBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let context =
            RenderContext::new().with_variable("name".to_string(), serde_json::json!("Ada"));
        block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: context,
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
    }

    #[test]
    fn test_handler_id_is_stable_edge() {
        assert_eq!(
            Handler::id(&handler(), IdRequest).expect("id ok").id,
            PROMPT_HANDLER_ID
        );
    }

    #[test]
    fn test_handler_pattern_is_stable_edge() {
        assert_eq!(
            Handler::pattern(&handler(), PatternRequest)
                .expect("pattern ok")
                .pattern,
            PROMPT_HANDLER_PATTERN
        );
    }

    #[test]
    fn test_handler_execute_missing_variable_error() {
        let security: SecurityContext = SecurityContext::unauthenticated();
        let commands = DirectCommandBus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = HandlerContext {
            security: &security,
            commands: &commands,
            observer: observer.as_ref(),
        };
        let empty_ctx = RenderContext::new();
        let result = block_on(Handler::execute(
            &handler(),
            ExecutionRequest {
                req: empty_ctx,
                ctx: &ctx,
            },
        ));
        assert!(result.is_err());
    }
}
