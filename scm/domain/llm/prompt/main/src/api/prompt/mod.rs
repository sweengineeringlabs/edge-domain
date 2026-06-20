//! Prompt domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::PromptError;
pub use traits::{ContextManager, Prompt, PromptBootstrap, TokenCounter};
pub use types::{
    HeuristicTokenCounter, MapContextManager, PromptCache, PromptCacheBuilder, PromptMetadata,
    PromptMetadataBuilder, RenderContext, StaticPrompt, StdPromptFactory, Variable, VariableBuilder,
    VariableType,
};
