//! [`EstimateTokensResponse`] — response for [`TokenCounter::estimate_tokens`](crate::api::prompt::traits::TokenCounter::estimate_tokens).

/// The estimated token count for the requested text.
#[derive(Debug, PartialEq)]
pub struct EstimateTokensResponse {
    /// Estimated number of tokens.
    pub count: usize,
}
