//! `ReasoningChainBuilder` — fluent builder for [`ReasoningChain`].

use crate::api::reasoning::types::{ReasoningChain, ThinkingProcess};

/// Fluent builder for [`ReasoningChain`].
#[derive(Clone, Debug)]
pub struct ReasoningChainBuilder {
    id: String,
    processes: Vec<ThinkingProcess>,
    is_complete: bool,
    final_answer: Option<String>,
}

impl ReasoningChainBuilder {
    /// Start a new builder for the chain with the given `id`.
    pub fn new(id: String) -> Self {
        Self {
            id,
            processes: vec![],
            is_complete: false,
            final_answer: None,
        }
    }

    /// Append a thinking process to the chain.
    pub fn process(mut self, process: ThinkingProcess) -> Self {
        self.processes.push(process);
        self
    }

    /// Mark the chain complete with a final answer.
    pub fn final_answer(mut self, value: String) -> Self {
        self.is_complete = true;
        self.final_answer = Some(value);
        self
    }

    /// Build the [`ReasoningChain`].
    pub fn build(self) -> ReasoningChain {
        let mut chain = ReasoningChain::new(self.id);
        for process in self.processes {
            chain.add_process(process);
        }
        chain.is_complete = self.is_complete;
        chain.final_answer = self.final_answer;
        chain
    }
}
