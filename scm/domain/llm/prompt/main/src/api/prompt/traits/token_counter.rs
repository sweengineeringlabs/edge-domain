//! `TokenCounter` — prompt tokenization contract.

use crate::api::prompt::errors::PromptError;
use crate::api::prompt::types::{
    CountTokensRequest, CountTokensResponse, EstimateTokensRequest, EstimateTokensResponse,
    ExactnessRequest, ExactnessResponse, TokenizerNameRequest, TokenizerNameResponse,
};

/// Token counting for prompts (exact or approximate tokenization).
pub trait TokenCounter: Send + Sync {
    /// Count tokens in the request text.
    fn count_tokens(&self, req: CountTokensRequest<'_>)
        -> Result<CountTokensResponse, PromptError>;

    /// Estimate tokens without full tokenization (faster, less precise).
    fn estimate_tokens(
        &self,
        req: EstimateTokensRequest<'_>,
    ) -> Result<EstimateTokensResponse, PromptError>;

    /// Name of the tokenizer/model (e.g. `"cl100k_base"`).
    fn tokenizer_name(
        &self,
        req: TokenizerNameRequest,
    ) -> Result<TokenizerNameResponse, PromptError>;

    /// Whether [`count_tokens`](TokenCounter::count_tokens) is exact rather than
    /// an estimate.
    fn is_exact(&self, req: ExactnessRequest) -> Result<ExactnessResponse, PromptError>;
}
