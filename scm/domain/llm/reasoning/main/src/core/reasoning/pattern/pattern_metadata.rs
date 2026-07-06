//! Constructors for [`PatternMetadata`].

use crate::api::{PatternMetadata, ReasoningPattern};

impl PatternMetadata {
    /// Create new pattern metadata with defaults
    pub fn new(pattern: ReasoningPattern) -> Self {
        Self {
            pattern,
            max_depth: Self::derive_max_depth(pattern),
            max_tokens: 8000,
            min_confidence: 0.6,
            allow_backtracking: pattern.is_iterative(),
            timeout_secs: 60,
            tags: vec![],
        }
    }

    /// Derive the default maximum reasoning depth from a pattern's expected step count.
    fn derive_max_depth(pattern: ReasoningPattern) -> usize {
        pattern.expected_step_count() as usize * 2
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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_defaults_from_pattern() {
        let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought);
        assert_eq!(meta.pattern, ReasoningPattern::ChainOfThought);
    }

    /// @covers: with_max_depth
    #[test]
    fn test_with_max_depth_overrides_default() {
        let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought).with_max_depth(10);
        assert_eq!(meta.max_depth, 10);
    }

    /// @covers: with_max_tokens
    #[test]
    fn test_with_max_tokens_overrides_default() {
        let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought).with_max_tokens(1000);
        assert_eq!(meta.max_tokens, 1000);
    }

    /// @covers: with_min_confidence
    #[test]
    fn test_with_min_confidence_clamps_above_one() {
        let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought).with_min_confidence(1.5);
        assert_eq!(meta.min_confidence, 1.0);
    }

    /// @covers: with_tag
    #[test]
    fn test_with_tag_avoids_duplicates() {
        let meta = PatternMetadata::new(ReasoningPattern::ChainOfThought)
            .with_tag("x".to_string())
            .with_tag("x".to_string());
        assert_eq!(meta.tags.len(), 1);
    }

    /// @covers: derive_max_depth
    #[test]
    fn test_derive_max_depth_doubles_expected_step_count() {
        let depth = PatternMetadata::derive_max_depth(ReasoningPattern::ChainOfThought);
        assert_eq!(
            depth,
            ReasoningPattern::ChainOfThought.expected_step_count() as usize * 2
        );
    }
}
