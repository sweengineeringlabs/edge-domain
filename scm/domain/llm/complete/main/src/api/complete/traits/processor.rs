//! `Processor` — SEA processor boundary for completion pipelines.

use async_trait::async_trait;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{CompletionRequest, CompletionResponse};

/// SEA processor boundary (service_type = `"processor"`).
///
/// Sits between the transport layer and the [`Completer`](crate::api::complete::traits::Completer):
/// implementations may rewrite requests, enforce policies, or fan out to multiple backends.
#[async_trait]
pub trait Processor: Send + Sync {
    /// Process a completion request and return the response.
    async fn process(
        &self,
        request: &CompletionRequest,
    ) -> Result<CompletionResponse, CompleteError>;
}
