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
