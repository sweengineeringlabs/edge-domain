//! [`CountTokensRequest`] — request for [`TokenCounter::count_tokens`](crate::api::prompt::traits::TokenCounter::count_tokens).

/// Request to count tokens in `text`.
#[derive(Debug, PartialEq)]
pub struct CountTokensRequest<'a> {
    /// The text to count tokens in.
    pub text: &'a str,
}
