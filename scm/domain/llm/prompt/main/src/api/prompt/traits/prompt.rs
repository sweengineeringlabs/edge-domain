//! `Prompt` — the template-rendering contract (primary trait).

use async_trait::async_trait;

use crate::api::prompt::errors::PromptError;
use crate::api::prompt::types::{PromptCache, PromptMetadata, RenderContext, VariableType};

/// Template-based prompt with variable substitution.
///
/// Decouples prompt construction from rendering logic (handlebars, Jinja2,
/// custom): callers depend on this contract and inject a concrete prompt.
#[async_trait]
pub trait Prompt: Send + Sync {
    /// Render the template against the given context.
    ///
    /// Returns [`PromptError::IncompleteContext`] when a required variable is
    /// absent from `context`.
    async fn render(&self, context: &RenderContext) -> Result<String, PromptError>;

    /// Metadata about this template (id, name, version, variables).
    fn metadata(&self) -> PromptMetadata;

    /// Validate template syntax (e.g. balanced placeholder braces).
    ///
    /// Returns [`PromptError::InvalidSyntax`] on malformed templates.
    fn validate(&self) -> Result<(), PromptError>;

    /// Declared type of the named variable, if the template declares it.
    fn variable_type(&self, name: &str) -> Option<VariableType>;

    /// Build a cache entry for an already-rendered prompt.
    fn cache(&self, context: &RenderContext, rendered: String) -> PromptCache;
}
