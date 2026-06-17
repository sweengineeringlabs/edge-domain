//! Prompt domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::PromptError;
pub use traits::{ContextManager, Prompt, PromptFactory, TokenCounter};
pub use types::{
    HeuristicTokenCounter, MapContextManager, PromptCache, PromptCacheBuilder, DefaultPrompt,
    PromptMetadata, PromptMetadataBuilder, RenderContext, StaticPrompt, StdPromptFactory, Variable,
    VariableBuilder, VariableType,
};
