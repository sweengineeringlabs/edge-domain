//! `TokenCounter` impl for `HeuristicTokenCounter`.

use crate::api::HeuristicTokenCounter;
use crate::api::TokenCounter;

impl TokenCounter for HeuristicTokenCounter {
    fn count_tokens(&self, text: &str) -> usize {
        if text.is_empty() {
            return 0;
        }
        // Count whitespace-delimited words, then refine by character budget so
        // long words contribute proportionally more tokens.
        let words = text.split_whitespace().count().max(1);
        let by_chars = text.chars().count().div_ceil(self.ratio());
        words.max(by_chars)
    }

    fn estimate_tokens(&self, text: &str) -> usize {
        text.chars().count().div_ceil(self.ratio())
    }

    fn tokenizer_name(&self) -> &'static str {
        Self::TOKENIZER_NAME
    }

    fn is_exact(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tokens_empty_is_zero() {
        assert_eq!(HeuristicTokenCounter::new().count_tokens(""), 0);
    }

    #[test]
    fn test_count_tokens_scales_with_length() {
        let counter = HeuristicTokenCounter::new();
        let short = counter.count_tokens("hi");
        let long = counter.count_tokens("a much longer piece of text here");
        assert!(long > short);
    }

    #[test]
    fn test_estimate_tokens_uses_char_ratio() {
        let counter = HeuristicTokenCounter::with_ratio(4);
        assert_eq!(counter.estimate_tokens("abcd"), 1);
    }

    #[test]
    fn test_is_exact_is_false() {
        assert!(!HeuristicTokenCounter::new().is_exact());
    }
}
