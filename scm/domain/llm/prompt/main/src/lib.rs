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

pub use saf::{
    ContextManager, HeuristicTokenCounter, MapContextManager, Prompt, PromptCache,
    PromptCacheBuilder, PromptEndpoint, PromptError, PromptFactory, PromptMetadata,
    PromptMetadataBuilder, RenderContext, StaticPrompt, StdPromptFactory, TokenCounter, Variable,
    VariableBuilder, VariableType, CONTEXT_MANAGER_SVC, PROMPT_FACTORY_SVC, PROMPT_SVC,
    TOKEN_COUNTER_SVC,
};
