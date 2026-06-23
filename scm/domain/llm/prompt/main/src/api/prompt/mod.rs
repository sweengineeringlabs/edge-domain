//! Prompt domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;

pub use errors::PromptError;
pub use traits::{ContextManager, Prompt, PromptBootstrap, TemplateProvider, TokenCounter};
pub use types::{
    CatalogTemplateProvider, HeuristicTokenCounter, MapContextManager, PromptCache,
    PromptMetadata, PromptTemplate,
    RenderContext, StaticPrompt, StdPromptFactory, Variable,
    VariableType,
};
