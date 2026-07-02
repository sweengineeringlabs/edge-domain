//! [`CountTokensResponse`] — response for [`TokenCounter::count_tokens`](crate::api::prompt::traits::TokenCounter::count_tokens).

/// The token count for the requested text.
#[derive(Debug, PartialEq)]
pub struct CountTokensResponse {
    /// Number of tokens counted.
    pub count: usize,
}
