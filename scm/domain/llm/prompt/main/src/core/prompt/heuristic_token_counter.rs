//! Constructors and `TokenCounter` impl for `HeuristicTokenCounter`.

use crate::api::HeuristicTokenCounter;
use crate::api::PromptError;
use crate::api::TokenCounter;
use crate::api::{
    CountTokensRequest, CountTokensResponse, EstimateTokensRequest, EstimateTokensResponse,
    ExactnessRequest, ExactnessResponse, TokenizerNameRequest, TokenizerNameResponse,
};

impl HeuristicTokenCounter {
    /// Average characters per token used by the heuristic.
    pub const DEFAULT_CHARS_PER_TOKEN: usize = 4;

    /// Stable identifier reported by this counter's tokenizer.
    pub const TOKENIZER_NAME: &'static str = "heuristic-chars";

    /// Construct a counter using the default characters-per-token ratio.
    pub fn new() -> Self {
        Self {
            chars_per_token: Self::DEFAULT_CHARS_PER_TOKEN,
        }
    }

    /// Construct a counter with a custom characters-per-token ratio.
    ///
    /// A ratio of zero is clamped to 1 to avoid division by zero.
    pub fn with_ratio(chars_per_token: usize) -> Self {
        Self {
            chars_per_token: chars_per_token.max(1),
        }
    }

    /// Effective characters-per-token ratio (never zero).
    pub(crate) fn ratio(&self) -> usize {
        self.chars_per_token.max(1)
    }
}

impl TokenCounter for HeuristicTokenCounter {
    fn count_tokens(
        &self,
        req: CountTokensRequest<'_>,
    ) -> Result<CountTokensResponse, PromptError> {
        if req.text.is_empty() {
            return Ok(CountTokensResponse { count: 0 });
        }
        // Count whitespace-delimited words, then refine by character budget so
        // long words contribute proportionally more tokens.
        let words = req.text.split_whitespace().count().max(1);
        let by_chars = req.text.chars().count().div_ceil(self.ratio());
        Ok(CountTokensResponse {
            count: words.max(by_chars),
        })
    }

    fn estimate_tokens(
        &self,
        req: EstimateTokensRequest<'_>,
    ) -> Result<EstimateTokensResponse, PromptError> {
        Ok(EstimateTokensResponse {
            count: req.text.chars().count().div_ceil(self.ratio()),
        })
    }

    fn tokenizer_name(
        &self,
        _req: TokenizerNameRequest,
    ) -> Result<TokenizerNameResponse, PromptError> {
        Ok(TokenizerNameResponse {
            name: Self::TOKENIZER_NAME,
        })
    }

    fn is_exact(&self, _req: ExactnessRequest) -> Result<ExactnessResponse, PromptError> {
        Ok(ExactnessResponse { exact: false })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_uses_default_chars_per_token() {
        let counter = HeuristicTokenCounter::new();
        assert_eq!(
            counter.chars_per_token,
            HeuristicTokenCounter::DEFAULT_CHARS_PER_TOKEN
        );
    }

    #[test]
    fn test_count_tokens_empty_is_zero() {
        let result = HeuristicTokenCounter::new()
            .count_tokens(CountTokensRequest { text: "" })
            .expect("count ok");
        assert_eq!(result.count, 0);
    }

    #[test]
    fn test_count_tokens_scales_with_length() {
        let counter = HeuristicTokenCounter::new();
        let short = counter
            .count_tokens(CountTokensRequest { text: "hi" })
            .expect("count ok")
            .count;
        let long = counter
            .count_tokens(CountTokensRequest {
                text: "a much longer piece of text here",
            })
            .expect("count ok")
            .count;
        assert!(long > short);
    }

    #[test]
    fn test_estimate_tokens_uses_char_ratio() {
        let counter = HeuristicTokenCounter::with_ratio(4);
        let result = counter
            .estimate_tokens(EstimateTokensRequest { text: "abcd" })
            .expect("estimate ok");
        assert_eq!(result.count, 1);
    }

    #[test]
    fn test_is_exact_is_false() {
        let result = HeuristicTokenCounter::new()
            .is_exact(ExactnessRequest)
            .expect("is_exact ok");
        assert!(!result.exact);
    }

    #[test]
    fn test_tokenizer_name_matches_constant() {
        let result = HeuristicTokenCounter::new()
            .tokenizer_name(TokenizerNameRequest)
            .expect("tokenizer_name ok");
        assert_eq!(result.name, HeuristicTokenCounter::TOKENIZER_NAME);
    }

    /// @covers: with_ratio
    #[test]
    fn test_with_ratio_clamps_zero_to_one() {
        let counter = HeuristicTokenCounter::with_ratio(0);
        assert_eq!(counter.ratio(), 1);
    }
}
