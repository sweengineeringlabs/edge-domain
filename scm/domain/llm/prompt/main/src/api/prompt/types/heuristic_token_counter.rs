//! `HeuristicTokenCounter` — reference [`TokenCounter`](crate::api::prompt::traits::TokenCounter) implementation.

/// Reference token counter that approximates token counts from character and
/// whitespace structure.
///
/// This is an estimator, not an exact tokenizer: it reports
/// [`is_exact`](crate::api::prompt::traits::TokenCounter::is_exact) as `false`.
#[derive(Clone, Copy, Debug, Default)]
pub struct HeuristicTokenCounter {
    pub(crate) chars_per_token: usize,
}
