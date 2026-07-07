//! Dispatchable-handler constructors for `StdPromptFactory`.

use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{Prompt, PromptMetadata, RenderContext, StaticPrompt, StdPromptFactory};
use crate::core::prompt::DefaultPromptHandler;

impl StdPromptFactory {
    /// Construct a dispatchable prompt handler backed by the given prompt.
    pub fn prompt_handler(
        prompt: Arc<dyn Prompt>,
    ) -> impl Handler<Request = RenderContext, Response = String> {
        DefaultPromptHandler { prompt }
    }

    /// Construct a dispatchable prompt handler backed by the reference [`StaticPrompt`].
    pub fn default_prompt_handler(
        template: String,
        metadata: PromptMetadata,
    ) -> impl Handler<Request = RenderContext, Response = String> {
        Self::prompt_handler(Self::to_static_prompt(template, metadata))
    }

    /// Lift a template body and metadata into a shared, type-erased [`Prompt`].
    fn to_static_prompt(template: String, metadata: PromptMetadata) -> Arc<dyn Prompt> {
        Arc::new(StaticPrompt::new(template, metadata))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{Variable, VariableKind};
    use edge_domain_command::DirectCommandBus;
    use edge_domain_handler::{ExecutionRequest, HandlerContext, IdRequest};
    use edge_domain_observer::StdObserveFactory;
    use edge_security_runtime::SecurityContext;
    use futures::executor::block_on;

    /// @covers: prompt_handler
    #[test]
    fn test_prompt_handler_renders_via_given_prompt() {
        let var = Variable::new("name".to_string(), VariableKind::String);
        let metadata =
            PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![var]);
        let prompt: Arc<dyn Prompt> =
            Arc::new(StaticPrompt::new("Hi {{name}}".to_string(), metadata));
        let handler = StdPromptFactory::prompt_handler(prompt);

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
            &handler,
            ExecutionRequest {
                req: context,
                ctx: &ctx,
            },
        ))
        .expect("handler ok");
        assert_eq!(out, "Hi Ada");
    }

    /// @covers: default_prompt_handler
    #[test]
    fn test_default_prompt_handler_has_stable_id() {
        let metadata =
            PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![]);
        let handler = StdPromptFactory::default_prompt_handler("static".to_string(), metadata);
        assert_eq!(
            Handler::id(&handler, IdRequest).expect("id ok").id,
            "prompt.render"
        );
    }

    /// @covers: to_static_prompt
    #[test]
    fn test_to_static_prompt_wraps_template_in_prompt() {
        let metadata =
            PromptMetadata::new("g".to_string(), "G".to_string(), "1".to_string(), vec![]);
        let prompt = StdPromptFactory::to_static_prompt("Hi".to_string(), metadata);
        let ctx = RenderContext::new();
        let out = block_on(prompt.render(crate::api::RenderRequest { context: &ctx }))
            .expect("render ok")
            .rendered;
        assert_eq!(out, "Hi");
    }
}
