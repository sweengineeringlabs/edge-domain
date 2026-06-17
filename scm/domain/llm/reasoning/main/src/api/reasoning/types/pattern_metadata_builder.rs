//! `PatternMetadataBuilder` — fluent builder for [`PatternMetadata`].

use crate::api::reasoning::types::{PatternMetadata, ReasoningPattern};

/// Fluent builder for [`PatternMetadata`].
#[derive(Clone, Debug)]
pub struct PatternMetadataBuilder {
    pattern: ReasoningPattern,
    max_depth: usize,
    max_tokens: usize,
    min_confidence: f32,
    allow_backtracking: bool,
    timeout_secs: u64,
    tags: Vec<String>,
}

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
        if !self.tags.contains(&value) {
            self.tags.push(value);
        }
        self
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
