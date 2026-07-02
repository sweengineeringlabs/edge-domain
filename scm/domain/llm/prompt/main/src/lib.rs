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
    ContextManager, Prompt, PromptBootstrap, TemplateProvider, TokenCounter, CONTEXT_MANAGER_SVC,
    PROMPT_FACTORY_SVC, PROMPT_SVC, TEMPLATE_PROVIDER_SVC, TOKEN_COUNTER_SVC,
};
pub use saf::{
    CONTEXT_MANAGER_SVC_FACTORY, PROMPT_FACTORY_SVC_FACTORY, PROMPT_SVC_FACTORY,
    TEMPLATE_PROVIDER_SVC_FACTORY, TOKEN_COUNTER_SVC_FACTORY,
};

// Re-export API value types for integration tests and client libraries
pub use api::{
    CatalogTemplateProvider, HeuristicTokenCounter, JsonValue, MapContextManager, PromptCache,
    PromptCacheBuilder, PromptError, PromptMetadata, PromptMetadataBuilder, PromptTemplate,
    PromptTemplateBuilder, RenderContext, StaticPrompt, StdPromptFactory, Variable,
    VariableBuilder, VariableKind,
};

// Re-export Request/Response envelope types for trait method calls
pub use api::{
    CacheBuildRequest, CacheBuildResponse, ClearVariablesRequest, CompletenessRequest,
    CompletenessResponse, ContextBuildRequest, ContextBuildResponse, CountTokensRequest,
    CountTokensResponse, EstimateTokensRequest, EstimateTokensResponse, ExactnessRequest,
    ExactnessResponse, ListByCategoryRequest, ListByCategoryResponse, ListTemplatesRequest,
    ListTemplatesResponse, PromptBootstrapNameRequest, PromptBootstrapNameResponse,
    PromptMetadataRequest, PromptMetadataResponse, PromptVariableKindRequest,
    PromptVariableKindResponse, RegisterVariableRequest, RenderRequest, RenderResponse,
    TemplateLookupRequest, TemplateLookupResponse, TemplateValidationRequest, TokenizerNameRequest,
    TokenizerNameResponse, VariableLookupRequest, VariableLookupResponse,
};
