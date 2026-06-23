//! # edge-llm-prompt
//!
//! LLM Prompt domain primitive: template rendering and dynamic context
//! management for composable prompts.
//!
//! Decouples prompt construction from rendering logic (handlebars, Jinja2,
//! custom). Public surface is delegated entirely through `saf/`.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

// Re-export SAF layer traits and factory markers
pub use saf::{
    ContextManager, Prompt,
    PromptBootstrap, TemplateProvider, TokenCounter,
    CONTEXT_MANAGER_SVC, PROMPT_FACTORY_SVC, PROMPT_SVC, TEMPLATE_PROVIDER_SVC, TOKEN_COUNTER_SVC,
};

// Re-export API value types for integration tests and client libraries
pub use api::{
    CatalogTemplateProvider, HeuristicTokenCounter, MapContextManager,
    PromptCache, PromptError, PromptMetadata,
    PromptTemplate, PromptTemplateBuilder, RenderContext, StaticPrompt,
    StdPromptFactory, Variable, VariableType,
};
