//! `ContextManager` — variable registration and context-building contract.

use crate::api::prompt::errors::PromptError;
use crate::api::prompt::types::{
    ClearVariablesRequest, CompletenessRequest, CompletenessResponse, ContextBuildRequest,
    ContextBuildResponse, RegisterVariableRequest, VariableLookupRequest, VariableLookupResponse,
};

/// Manages context variables and template-variable resolution.
pub trait ContextManager: Send + Sync {
    /// Register a variable under `name`.
    ///
    /// Returns [`PromptError::InvalidValue`] when `name` is empty.
    fn register_variable(&mut self, req: RegisterVariableRequest<'_>) -> Result<(), PromptError>;

    /// Get a registered variable by name.
    fn get_variable(
        &self,
        req: VariableLookupRequest<'_>,
    ) -> Result<VariableLookupResponse<'_>, PromptError>;

    /// Build a render context from the registered, satisfied variables.
    ///
    /// Returns [`PromptError::IncompleteContext`] when a required variable has
    /// neither a value nor a default.
    fn build_context(&self, req: ContextBuildRequest) -> Result<ContextBuildResponse, PromptError>;

    /// Remove all registered variables.
    fn clear(&mut self, req: ClearVariablesRequest) -> Result<(), PromptError>;

    /// Whether all required (non-default) variables are satisfied.
    fn is_complete(&self, req: CompletenessRequest) -> Result<CompletenessResponse, PromptError>;
}
