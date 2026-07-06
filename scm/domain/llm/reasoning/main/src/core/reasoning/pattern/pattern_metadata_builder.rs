//! Constructors and builder methods for [`PatternMetadataBuilder`].

use crate::api::{PatternMetadata, PatternMetadataBuilder, ReasoningPattern};

impl PatternMetadataBuilder {
    /// Start a new builder seeded from the pattern defaults.
    pub fn new(pattern: ReasoningPattern) -> Self {
        let defaults = PatternMetadata::new(pattern);
        Self {
            pattern,
            max_depth: defaults.max_depth,
            max_tokens: defaults.max_tokens,
            min_confidence: defaults.min_confidence,
            allow_backtracking: defaults.allow_backtracking,
            timeout_secs: defaults.timeout_secs,
            tags: defaults.tags,
        }
    }

    /// Set the maximum reasoning depth.
    pub fn max_depth(mut self, value: usize) -> Self {
        self.max_depth = value;
        self
    }

    /// Set the maximum token budget.
    pub fn max_tokens(mut self, value: usize) -> Self {
        self.max_tokens = value;
        self
    }

    /// Set the minimum confidence threshold (clamped to `0.0..=1.0`).
    pub fn min_confidence(mut self, value: f32) -> Self {
        self.min_confidence = value.clamp(0.0, 1.0);
        self
    }

    /// Set whether backtracking is allowed.
    pub fn allow_backtracking(mut self, value: bool) -> Self {
        self.allow_backtracking = value;
        self
    }

    /// Set the timeout in seconds.
    pub fn timeout_secs(mut self, value: u64) -> Self {
        self.timeout_secs = value;
        self
    }

    /// Add a categorization tag.
    pub fn tag(mut self, value: String) -> Self {
        Self::push_unique_tag(&mut self.tags, value);
        self
    }

    /// Push `value` onto `tags` unless it is already present.
    fn push_unique_tag(tags: &mut Vec<String>, value: String) {
        if !tags.contains(&value) {
            tags.push(value);
        }
    }

    /// Build the [`PatternMetadata`].
    pub fn build(self) -> PatternMetadata {
        PatternMetadata {
            pattern: self.pattern,
            max_depth: self.max_depth,
            max_tokens: self.max_tokens,
            min_confidence: self.min_confidence,
            allow_backtracking: self.allow_backtracking,
            timeout_secs: self.timeout_secs,
            tags: self.tags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_seeds_from_pattern_defaults() {
        let builder = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought);
        assert_eq!(builder.pattern, ReasoningPattern::ChainOfThought);
    }

    /// @covers: max_depth
    #[test]
    fn test_max_depth_overrides_value() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .max_depth(10)
            .build();
        assert_eq!(meta.max_depth, 10);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_pattern_metadata() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .tag("x".to_string())
            .build();
        assert_eq!(meta.tags, vec!["x".to_string()]);
    }

    /// @covers: tag
    #[test]
    fn test_tag_adds_to_list() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .tag("y".to_string())
            .build();
        assert!(meta.tags.contains(&"y".to_string()));
    }

    /// @covers: push_unique_tag
    #[test]
    fn test_push_unique_tag_skips_duplicate() {
        let mut tags = vec!["x".to_string()];
        PatternMetadataBuilder::push_unique_tag(&mut tags, "x".to_string());
        assert_eq!(tags.len(), 1);
    }

    /// @covers: max_tokens
    #[test]
    fn test_max_tokens_overrides_value() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .max_tokens(1234)
            .build();
        assert_eq!(meta.max_tokens, 1234);
    }

    /// @covers: min_confidence
    #[test]
    fn test_min_confidence_clamps_above_one() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .min_confidence(1.5)
            .build();
        assert_eq!(meta.min_confidence, 1.0);
    }

    /// @covers: allow_backtracking
    #[test]
    fn test_allow_backtracking_overrides_value() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .allow_backtracking(true)
            .build();
        assert!(meta.allow_backtracking);
    }

    /// @covers: timeout_secs
    #[test]
    fn test_timeout_secs_overrides_value() {
        let meta = PatternMetadataBuilder::new(ReasoningPattern::ChainOfThought)
            .timeout_secs(120)
            .build();
        assert_eq!(meta.timeout_secs, 120);
    }
}
