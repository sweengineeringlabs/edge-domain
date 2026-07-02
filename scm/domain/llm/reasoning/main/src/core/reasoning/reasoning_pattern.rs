//! Behaviour for [`ReasoningPattern`].

use crate::api::ReasoningPattern;

impl ReasoningPattern {
    /// Check if this pattern requires iterative refinement
    pub fn is_iterative(&self) -> bool {
        self.matches_one_of(&[
            ReasoningPattern::Reflection,
            ReasoningPattern::TreeOfThought,
        ])
    }

    /// Check if this pattern involves multiple agents/perspectives
    pub fn is_collaborative(&self) -> bool {
        self.matches_one_of(&[ReasoningPattern::MultiAgent])
    }

    /// Check if this pattern is one of `patterns`, shared by the classification checks.
    fn matches_one_of(&self, patterns: &[ReasoningPattern]) -> bool {
        patterns.contains(self)
    }

    /// Get human-readable pattern name
    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningPattern::ChainOfThought => "Chain of Thought",
            ReasoningPattern::TreeOfThought => "Tree of Thought",
            ReasoningPattern::Reflection => "Reflection",
            ReasoningPattern::FewShot => "Few-Shot Learning",
            ReasoningPattern::MultiAgent => "Multi-Agent",
            ReasoningPattern::Hierarchical => "Hierarchical Decomposition",
            ReasoningPattern::GraphBased => "Graph-Based",
        }
    }

    /// Expected number of reasoning steps (rough estimate)
    pub fn expected_step_count(&self) -> u32 {
        match self {
            ReasoningPattern::ChainOfThought => 3,
            ReasoningPattern::TreeOfThought => 7,
            ReasoningPattern::Reflection => 5,
            ReasoningPattern::FewShot => 2,
            ReasoningPattern::MultiAgent => 4,
            ReasoningPattern::Hierarchical => 6,
            ReasoningPattern::GraphBased => 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_iterative
    #[test]
    fn test_is_iterative_true_for_reflection() {
        assert!(ReasoningPattern::Reflection.is_iterative());
    }

    /// @covers: is_iterative
    #[test]
    fn test_is_iterative_false_for_chain_of_thought() {
        assert!(!ReasoningPattern::ChainOfThought.is_iterative());
    }

    /// @covers: is_collaborative
    #[test]
    fn test_is_collaborative_true_for_multi_agent() {
        assert!(ReasoningPattern::MultiAgent.is_collaborative());
    }

    /// @covers: as_str
    #[test]
    fn test_as_str_matches_expected_name() {
        assert_eq!(
            ReasoningPattern::ChainOfThought.as_str(),
            "Chain of Thought"
        );
    }

    /// @covers: expected_step_count
    #[test]
    fn test_expected_step_count_matches_pattern() {
        assert_eq!(ReasoningPattern::ChainOfThought.expected_step_count(), 3);
    }

    /// @covers: matches_one_of
    #[test]
    fn test_matches_one_of_checks_membership_in_list() {
        assert!(ReasoningPattern::Reflection.matches_one_of(&[ReasoningPattern::Reflection]));
        assert!(!ReasoningPattern::Reflection.matches_one_of(&[ReasoningPattern::MultiAgent]));
    }
}
