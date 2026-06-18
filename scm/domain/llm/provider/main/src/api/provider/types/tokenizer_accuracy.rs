use serde::{Deserialize, Serialize};

/// Accuracy level of token counting
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TokenizerAccuracy {
    /// Exact count from official tokenizer
    #[serde(rename = "exact")]
    Exact,

    /// Approximate count (within 5%)
    #[serde(rename = "approximate")]
    Approximate,

    /// Fallback estimate (rough heuristic)
    #[serde(rename = "fallback")]
    Fallback,
}
