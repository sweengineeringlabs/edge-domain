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
