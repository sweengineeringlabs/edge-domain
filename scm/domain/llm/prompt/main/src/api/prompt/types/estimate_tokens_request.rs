//! [`EstimateTokensRequest`] — request for [`TokenCounter::estimate_tokens`](crate::api::prompt::traits::TokenCounter::estimate_tokens).

/// Request to estimate tokens in `text` without full tokenization.
#[derive(Debug, PartialEq)]
pub struct EstimateTokensRequest<'a> {
    /// The text to estimate tokens for.
    pub text: &'a str,
}
