use serde::{Deserialize, Serialize};

/// Multi-strategy reasoning pattern for problem decomposition
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ReasoningPattern {
    /// Linear step-by-step reasoning
    #[serde(rename = "chain_of_thought")]
    ChainOfThought,

    /// Tree-based exploration of reasoning branches
    #[serde(rename = "tree_of_thought")]
    TreeOfThought,

    /// Self-reflection and correction
    #[serde(rename = "reflection")]
    Reflection,

    /// Analogical reasoning from examples
    #[serde(rename = "few_shot")]
    FewShot,

    /// Collaborative multi-agent reasoning
    #[serde(rename = "multi_agent")]
    MultiAgent,

    /// Hierarchical decomposition
    #[serde(rename = "hierarchical")]
    Hierarchical,

    /// Graph-based reasoning (knowledge graphs, dependency graphs)
    #[serde(rename = "graph_based")]
    GraphBased,
}

impl ReasoningPattern {
    /// Check if this pattern requires iterative refinement
    pub fn is_iterative(&self) -> bool {
        matches!(
            self,
            ReasoningPattern::Reflection | ReasoningPattern::TreeOfThought
        )
    }

    /// Check if this pattern involves multiple agents/perspectives
    pub fn is_collaborative(&self) -> bool {
        matches!(self, ReasoningPattern::MultiAgent)
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
