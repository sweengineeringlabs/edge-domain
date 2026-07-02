//! [`TokenizerNameRequest`] — request for [`TokenCounter::tokenizer_name`](crate::api::prompt::traits::TokenCounter::tokenizer_name).

/// Request for the tokenizer/model name. Carries no data.
#[derive(Debug, PartialEq)]
pub struct TokenizerNameRequest;
