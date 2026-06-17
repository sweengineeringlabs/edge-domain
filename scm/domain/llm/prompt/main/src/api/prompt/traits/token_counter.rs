//! `TokenCounter` — prompt tokenization contract.

/// Token counting for prompts (exact or approximate tokenization).
pub trait TokenCounter: Send + Sync {
    /// Count tokens in `text`.
    fn count_tokens(&self, text: &str) -> usize;

    /// Estimate tokens without full tokenization (faster, less precise).
    fn estimate_tokens(&self, text: &str) -> usize;

    /// Name of the tokenizer/model (e.g. `"cl100k_base"`).
    fn tokenizer_name(&self) -> &'static str;

    /// Whether [`count_tokens`](TokenCounter::count_tokens) is exact rather than
    /// an estimate.
    fn is_exact(&self) -> bool;
}
