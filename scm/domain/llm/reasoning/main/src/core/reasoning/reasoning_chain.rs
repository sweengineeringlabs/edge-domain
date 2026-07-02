//! Constructors and accessors for [`ReasoningChain`].

use crate::api::{ReasoningChain, ThinkingProcess};

impl ReasoningChain {
    /// Create a new reasoning chain
    pub fn new(id: String) -> Self {
        Self {
            id,
            processes: vec![],
            is_complete: false,
            final_answer: None,
        }
    }

    /// Add a thinking process to the chain
    pub fn add_process(&mut self, process: ThinkingProcess) {
        self.processes.push(process);
    }

    /// Get the number of processes in chain
    pub fn process_count(&self) -> usize {
        self.processes.len()
    }

    /// Get total reasoning steps across all processes
    pub fn total_step_count(&self) -> usize {
        self.processes.iter().map(|p| p.step_count()).sum()
    }

    /// Get total tokens consumed across chain
    pub fn total_tokens(&self) -> usize {
        self.processes.iter().map(|p| p.total_tokens).sum()
    }

    /// Mark chain as complete with final answer
    pub fn complete(mut self, answer: String) -> Self {
        self.is_complete = true;
        self.final_answer = Some(answer);
        self
    }

    /// Get average confidence across all steps
    pub fn average_confidence(&self) -> f32 {
        if self.processes.is_empty() {
            return 0.0;
        }
        Self::sum_confidence(&self.processes) / self.processes.len() as f32
    }

    /// Sum the per-process average confidence across `processes`.
    fn sum_confidence(processes: &[ThinkingProcess]) -> f32 {
        processes.iter().map(|p| p.average_confidence()).sum()
    }

    /// Check if all processes are complete
    pub fn all_complete(&self) -> bool {
        self.processes.iter().all(|p| p.is_complete)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::ReasoningStep;

    /// @covers: new
    #[test]
    fn test_new_starts_empty() {
        let chain = ReasoningChain::new("c1".to_string());
        assert_eq!(chain.process_count(), 0);
    }

    /// @covers: add_process
    #[test]
    fn test_add_process_increments_count() {
        let mut chain = ReasoningChain::new("c1".to_string());
        chain.add_process(ThinkingProcess::new("p1".to_string(), "q".to_string()));
        assert_eq!(chain.process_count(), 1);
    }

    /// @covers: total_tokens
    #[test]
    fn test_total_tokens_sums_processes() {
        let mut chain = ReasoningChain::new("c1".to_string());
        let mut p = ThinkingProcess::new("p1".to_string(), "q".to_string());
        p.total_tokens = 5;
        chain.add_process(p);
        assert_eq!(chain.total_tokens(), 5);
    }

    /// @covers: complete
    #[test]
    fn test_complete_sets_final_answer() {
        let chain = ReasoningChain::new("c1".to_string()).complete("done".to_string());
        assert_eq!(chain.final_answer, Some("done".to_string()));
    }

    /// @covers: average_confidence
    #[test]
    fn test_average_confidence_zero_when_empty() {
        let chain = ReasoningChain::new("c1".to_string());
        assert_eq!(chain.average_confidence(), 0.0);
    }

    /// @covers: all_complete
    #[test]
    fn test_all_complete_true_when_no_processes() {
        let chain = ReasoningChain::new("c1".to_string());
        assert!(chain.all_complete());
    }

    /// @covers: sum_confidence
    #[test]
    fn test_sum_confidence_sums_process_averages() {
        let mut p1 = ThinkingProcess::new("p1".to_string(), "q1".to_string());
        p1.add_step(
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_confidence(0.8),
        );
        let mut p2 = ThinkingProcess::new("p2".to_string(), "q2".to_string());
        p2.add_step(
            ReasoningStep::new(0, "y".to_string(), "analysis".to_string()).with_confidence(0.4),
        );

        let total = ReasoningChain::sum_confidence(&[p1, p2]);
        assert_eq!(total, 0.8_f32 + 0.4_f32);
    }

    /// @covers: process_count
    #[test]
    fn test_process_count_reflects_added_processes() {
        let mut chain = ReasoningChain::new("c1".to_string());
        chain.add_process(ThinkingProcess::new("p1".to_string(), "q".to_string()));
        chain.add_process(ThinkingProcess::new("p2".to_string(), "q".to_string()));
        assert_eq!(chain.process_count(), 2);
    }

    /// @covers: total_step_count
    #[test]
    fn test_total_step_count_sums_across_processes() {
        let mut chain = ReasoningChain::new("c1".to_string());
        let mut p = ThinkingProcess::new("p1".to_string(), "q".to_string());
        p.add_step(ReasoningStep::new(
            0,
            "x".to_string(),
            "analysis".to_string(),
        ));
        chain.add_process(p);
        assert_eq!(chain.total_step_count(), 1);
    }
}
