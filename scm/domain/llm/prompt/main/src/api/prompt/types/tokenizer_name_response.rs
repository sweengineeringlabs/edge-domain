//! [`TokenizerNameResponse`] — response for [`TokenCounter::tokenizer_name`](crate::api::prompt::traits::TokenCounter::tokenizer_name).

/// The name of the tokenizer/model.
#[derive(Debug, PartialEq)]
pub struct TokenizerNameResponse {
    /// Stable identifier for the tokenizer/model (e.g. `"cl100k_base"`).
    pub name: &'static str,
}
