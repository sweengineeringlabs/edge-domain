use std::sync::Arc;

use edge_domain_handler::Handler;

use crate::api::{Prompt, PromptMetadata, RenderContext, StaticPrompt};
use crate::core::prompt::default_prompt::DefaultPromptHandler;

pub use crate::api::PromptCacheBuilder;
pub use crate::api::PromptFactory;
pub use crate::api::PromptMetadataBuilder;
pub use crate::api::StdPromptFactory;
pub use crate::api::VariableBuilder;

/// SAF contract identifier for the prompt-factory service.
pub const PROMPT_FACTORY_SVC: &str = "prompt_factory";

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
    prompt_handler(Arc::new(StaticPrompt::new(template, metadata)))
}
