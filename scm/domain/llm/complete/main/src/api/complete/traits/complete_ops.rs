//! `CompleteOps` — assembly and inspection helpers for completion payloads.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{
    CompletionRequest, CompletionResponse, FinishReason, Message, TokenUsage,
};

/// Assembly and inspection contract for completion payloads.
pub trait CompleteOps: Send + Sync {
    /// Build a [`CompletionRequest`] from a model id and message list.
    fn assemble(model: String, messages: Vec<Message>) -> CompletionRequest
    where
        Self: Sized,
    {
        CompletionRequest::new(model, messages)
    }

    /// Extract the token usage from a response.
    fn extract_usage(resp: &CompletionResponse) -> &TokenUsage
    where
        Self: Sized,
    {
        &resp.usage
    }

    /// Extract the finish reason from a response.
    fn extract_finish(resp: &CompletionResponse) -> &FinishReason
    where
        Self: Sized,
    {
        &resp.finish_reason
    }

    /// Construct an empty response shell with the given id and model.
    fn create_response(id: String, model: String) -> CompletionResponse
    where
        Self: Sized,
    {
        CompletionResponse { id, model, ..Default::default() }
    }

    /// Construct a zeroed token usage record.
    fn create_usage() -> TokenUsage
    where
        Self: Sized,
    {
        TokenUsage::default()
    }

    /// Validate the request and return an error if it is structurally invalid.
    fn check(&self, request: &CompletionRequest) -> Result<(), CompleteError>;
}
