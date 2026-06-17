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

impl PatternMetadata {
    /// Create new pattern metadata with defaults
    pub fn new(pattern: ReasoningPattern) -> Self {
        Self {
            pattern,
            max_depth: pattern.expected_step_count() as usize * 2,
            max_tokens: 8000,
            min_confidence: 0.6,
            allow_backtracking: pattern.is_iterative(),
            timeout_secs: 60,
            tags: vec![],
        }
    }

    /// Set maximum depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Set maximum tokens
    pub fn with_max_tokens(mut self, tokens: usize) -> Self {
        self.max_tokens = tokens;
        self
    }

    /// Set minimum confidence threshold
    pub fn with_min_confidence(mut self, confidence: f32) -> Self {
        self.min_confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Add a tag
    pub fn with_tag(mut self, tag: String) -> Self {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
        self
    }
}
