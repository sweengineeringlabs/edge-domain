use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{Prompt, PromptMetadata, RenderContext, StaticPrompt, StdPromptFactory};
use crate::spi::DefaultPromptHandler;

pub use crate::api::PromptBootstrap;

/// SAF contract identifier for the prompt-factory service.
pub const PROMPT_FACTORY_SVC: &str = "prompt_factory";

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
        Self::prompt_handler(Arc::new(StaticPrompt::new(template, metadata)))
    }
}
