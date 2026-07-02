//! Constructors and builder methods for [`ReasoningChainBuilder`].

use crate::api::{ReasoningChain, ReasoningChainBuilder, ThinkingProcess};

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
        Self::apply_completion(&mut chain, self.is_complete, self.final_answer);
        chain
    }

    /// Apply the builder's completion state onto an assembled `chain`.
    fn apply_completion(
        chain: &mut ReasoningChain,
        is_complete: bool,
        final_answer: Option<String>,
    ) {
        chain.is_complete = is_complete;
        chain.final_answer = final_answer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_empty() {
        let builder = ReasoningChainBuilder::new("c1".to_string());
        assert_eq!(builder.id, "c1");
    }

    /// @covers: process
    #[test]
    fn test_process_appends_to_list() {
        let builder = ReasoningChainBuilder::new("c1".to_string())
            .process(ThinkingProcess::new("p1".to_string(), "q".to_string()));
        assert_eq!(builder.processes.len(), 1);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_reasoning_chain() {
        let chain = ReasoningChainBuilder::new("c1".to_string())
            .final_answer("done".to_string())
            .build();
        assert_eq!(chain.final_answer, Some("done".to_string()));
    }

    /// @covers: apply_completion
    #[test]
    fn test_apply_completion_sets_fields() {
        let mut chain = ReasoningChain::new("c1".to_string());
        ReasoningChainBuilder::apply_completion(&mut chain, true, Some("done".to_string()));
        assert!(chain.is_complete);
        assert_eq!(chain.final_answer, Some("done".to_string()));
    }

    /// @covers: final_answer
    #[test]
    fn test_final_answer_marks_builder_complete() {
        let builder = ReasoningChainBuilder::new("c1".to_string()).final_answer("done".to_string());
        assert!(builder.is_complete);
    }
}
