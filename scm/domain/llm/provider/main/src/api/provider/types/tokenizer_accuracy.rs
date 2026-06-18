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

#[cfg(test)]
mod tests {
    use super::TokenizerAccuracy;

    #[test]
    fn test_tokenizer_accuracy_variants_distinct() {
        assert_ne!(TokenizerAccuracy::Exact, TokenizerAccuracy::Approximate);
        assert_ne!(TokenizerAccuracy::Approximate, TokenizerAccuracy::Fallback);
    }

    #[test]
    fn test_tokenizer_accuracy_equality() {
        assert_eq!(TokenizerAccuracy::Exact, TokenizerAccuracy::Exact);
    }

    #[test]
    fn test_tokenizer_accuracy_serde_roundtrip() {
        let json = serde_json::to_string(&TokenizerAccuracy::Fallback).expect("serialize");
        let back: TokenizerAccuracy = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, TokenizerAccuracy::Fallback);
    }
}
