//! Prompt domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::PromptError;
pub use traits::{ContextManager, Prompt, PromptBootstrap, TemplateProvider, TokenCounter};
pub use types::{
    HeuristicTokenCounter, InMemoryTemplateProvider, MapContextManager, PromptCache,
    PromptCacheBuilder, PromptMetadata, PromptMetadataBuilder, PromptTemplate,
    PromptTemplateBuilder, RenderContext, StaticPrompt, StdPromptFactory, Variable,
    VariableBuilder, VariableType,
};
