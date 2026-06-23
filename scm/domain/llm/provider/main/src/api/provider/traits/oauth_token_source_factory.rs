//! [`OAuthTokenSourceFactory`] — plugin-provided OAuth token source initialization.

use std::path::Path;
use std::sync::Arc;

/// Factory for initializing provider-specific OAuth token sources.
///
/// Implemented by each LLM provider plugin to handle provider-specific token source
/// initialization (e.g., Claude vs. OpenAI, Anthropic vs. Google). From ADR-015 Tier 2a
/// egress security infrastructure.
///
/// Plugins instantiate their own token source types (e.g., `ClaudeTokenSource`) and return
/// them as trait objects. The framework handles credential resolution; the plugin handles
/// token source initialization from resolved credentials.
pub trait OAuthTokenSourceFactory: Send + Sync {
    /// Initialize an OAuth token source from a credential file path.
    ///
    /// Called by provider factories after credential source resolution (from framework).
    /// Plugin returns its concrete token source type, opaque to the caller.
    ///
    /// # Errors
    ///
    /// Returns `Err` if the credential file is inaccessible, malformed, or cannot
    /// produce a valid token source.
    fn create_from_file(&self, path: &Path) -> Result<Arc<dyn std::any::Any>, String>;
}
