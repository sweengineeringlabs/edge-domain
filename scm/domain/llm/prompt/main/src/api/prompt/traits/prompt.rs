//! `Prompt` — the template-rendering contract (primary trait).

use async_trait::async_trait;

use crate::api::prompt::errors::PromptError;
use crate::api::prompt::types::{
    CacheBuildRequest, CacheBuildResponse, PromptMetadataRequest, PromptMetadataResponse,
    PromptVariableKindRequest, PromptVariableKindResponse, RenderRequest, RenderResponse,
    TemplateValidationRequest,
};

/// Template-based prompt with variable substitution.
///
/// Decouples prompt construction from rendering logic (handlebars, Jinja2,
/// custom): callers depend on this contract and inject a concrete prompt.
#[async_trait]
pub trait Prompt: Send + Sync {
    /// Render the template against the given context.
    ///
    /// Returns [`PromptError::IncompleteContext`] when a required variable is
    /// absent from the context.
    async fn render(&self, req: RenderRequest<'_>) -> Result<RenderResponse, PromptError>;

    /// Metadata about this template (id, name, version, variables).
    fn metadata(&self, req: PromptMetadataRequest) -> Result<PromptMetadataResponse, PromptError>;

    /// Validate template syntax (e.g. balanced placeholder braces).
    ///
    /// Returns [`PromptError::InvalidSyntax`] on malformed templates.
    fn validate(&self, req: TemplateValidationRequest) -> Result<(), PromptError>;

    /// Declared type of the named variable, if the template declares it.
    fn variable_kind(
        &self,
        req: PromptVariableKindRequest<'_>,
    ) -> Result<PromptVariableKindResponse, PromptError>;

    /// Build a cache entry for an already-rendered prompt.
    fn cache(&self, req: CacheBuildRequest<'_>) -> Result<CacheBuildResponse, PromptError>;
}
