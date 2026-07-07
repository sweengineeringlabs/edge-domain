//! Prompt domain contracts: traits, value types, and errors.

pub mod errors;
pub mod traits;
pub mod types;
pub(crate) mod variable;

pub use errors::PromptError;
pub use traits::{ContextManager, Prompt, TemplateProvider, TokenCounter};
pub use types::{
    CacheBuildRequest, CacheBuildResponse, ClearVariablesRequest, CompletenessRequest,
    CompletenessResponse, ContextBuildRequest, ContextBuildResponse, CountTokensRequest,
    CountTokensResponse, EstimateTokensRequest, EstimateTokensResponse, ExactnessRequest,
    ExactnessResponse, ListByCategoryRequest, ListByCategoryResponse, ListTemplatesRequest,
    ListTemplatesResponse,
    PromptMetadataRequest, PromptMetadataResponse, PromptVariableKindRequest,
    PromptVariableKindResponse, RegisterVariableRequest, RenderRequest, RenderResponse,
    TemplateLookupRequest, TemplateLookupResponse, TemplateValidationRequest, TokenizerNameRequest,
    TokenizerNameResponse, VariableLookupRequest, VariableLookupResponse,
};
pub use types::{
    CatalogTemplateProvider, HeuristicTokenCounter, JsonValue, MapContextManager, PromptCache,
    PromptCacheBuilder, PromptMetadata, PromptMetadataBuilder, PromptTemplate,
    PromptTemplateBuilder, RenderContext, StaticPrompt, StdPromptFactory, Variable,
    VariableBuilder, VariableKind,
};
