//! [`OauthTokenSourceResolver`] — plugin-provided OAuth token source initialization.

use crate::api::provider::errors::OauthTokenSourceError;
use crate::api::provider::types::{TokenSourceFileRequest, TokenSourceInitResponse};

/// Resolver for initializing provider-specific OAuth token sources.
///
/// Implemented by each LLM provider plugin to handle provider-specific token source
/// initialization (e.g., Claude vs. OpenAI, Anthropic vs. Google). From ADR-015 Tier 2a
/// egress security infrastructure.
///
/// Plugins instantiate their own token source types (e.g., `ClaudeTokenSource`) and return
/// them as trait objects. The framework handles credential resolution; the plugin handles
/// token source initialization from resolved credentials.
pub trait OauthTokenSourceResolver: Send + Sync {
    /// Initialize an OAuth token source from a credential file path.
    ///
    /// Called by provider factories after credential source resolution (from framework).
    /// Plugin returns its concrete token source type, opaque to the caller.
    ///
    /// # Errors
    ///
    /// Returns [`OauthTokenSourceError`] if the credential file is inaccessible, malformed, or
    /// cannot produce a valid token source.
    fn create_from_file(
        &self,
        req: TokenSourceFileRequest<'_>,
    ) -> Result<TokenSourceInitResponse, OauthTokenSourceError>;
}
