use crate::api::reasoning::types::ReasoningPattern;
use serde::{Deserialize, Serialize};

/// Metadata about a reasoning pattern configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PatternMetadata {
    /// Pattern type
    pub pattern: ReasoningPattern,

    /// Maximum reasoning depth
    pub max_depth: usize,

    /// Maximum tokens for reasoning
    pub max_tokens: usize,

    /// Minimum confidence threshold for step success
    pub min_confidence: f32,

    /// Whether to allow backtracking
    pub allow_backtracking: bool,

    /// Timeout in seconds
    pub timeout_secs: u64,

    /// Tags for categorization
    pub tags: Vec<String>,
}
