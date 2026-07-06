//! Constructors and accessors for [`ThinkingProcess`].

use crate::api::{ReasoningStep, ThinkingProcess};

impl ThinkingProcess {
    /// Create a new thinking process
    pub fn new(id: String, problem: String) -> Self {
        Self {
            id,
            problem,
            steps: vec![],
            total_tokens: 0,
            is_complete: false,
            conclusion: None,
        }
    }

    /// Add a reasoning step
    pub fn add_step(&mut self, step: ReasoningStep) {
        self.total_tokens += step.tokens_consumed;
        self.steps.push(step);
    }

    /// Get the number of reasoning steps
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get average confidence across all steps
    pub fn average_confidence(&self) -> f32 {
        if self.steps.is_empty() {
            return 0.0;
        }
        self.steps.iter().map(|s| s.confidence).sum::<f32>() / self.steps.len() as f32
    }

    /// Mark process as complete with conclusion
    pub fn complete(mut self, conclusion: String) -> Self {
        self.is_complete = true;
        self.conclusion = Some(conclusion);
        self
    }

    /// Get high-confidence steps
    pub fn confident_steps(&self) -> Vec<&ReasoningStep> {
        self.filter_by_confidence(true)
    }

    /// Get low-confidence steps (potential weak points)
    pub fn uncertain_steps(&self) -> Vec<&ReasoningStep> {
        self.filter_by_confidence(false)
    }

    /// Select steps whose [`ReasoningStep::is_confident`] matches `confident` —
    /// shared by [`Self::confident_steps`] and [`Self::uncertain_steps`].
    fn filter_by_confidence(&self, confident: bool) -> Vec<&ReasoningStep> {
        self.steps
            .iter()
            .filter(|s| s.is_confident() == confident)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_empty() {
        let process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        assert_eq!(process.step_count(), 0);
    }

    /// @covers: add_step
    #[test]
    fn test_add_step_accumulates_tokens() {
        let mut process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        process.add_step(
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_tokens(5),
        );
        assert_eq!(process.total_tokens, 5);
    }

    /// @covers: average_confidence
    #[test]
    fn test_average_confidence_zero_when_empty() {
        let process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        assert_eq!(process.average_confidence(), 0.0);
    }

    /// @covers: complete
    #[test]
    fn test_complete_sets_conclusion() {
        let process =
            ThinkingProcess::new("p1".to_string(), "q".to_string()).complete("done".to_string());
        assert_eq!(process.conclusion, Some("done".to_string()));
    }

    /// @covers: confident_steps
    #[test]
    fn test_confident_steps_filters_by_threshold() {
        let mut process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        process.add_step(
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_confidence(0.9),
        );
        assert_eq!(process.confident_steps().len(), 1);
    }

    /// @covers: uncertain_steps
    #[test]
    fn test_uncertain_steps_filters_by_threshold() {
        let mut process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        process.add_step(
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_confidence(0.2),
        );
        assert_eq!(process.uncertain_steps().len(), 1);
    }

    /// @covers: filter_by_confidence
    #[test]
    fn test_filter_by_confidence_splits_steps_both_ways() {
        let mut process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        process.add_step(
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_confidence(0.9),
        );
        process.add_step(
            ReasoningStep::new(1, "y".to_string(), "analysis".to_string()).with_confidence(0.2),
        );
        assert_eq!(process.filter_by_confidence(true).len(), 1);
        assert_eq!(process.filter_by_confidence(false).len(), 1);
    }

    /// @covers: step_count
    #[test]
    fn test_step_count_reflects_added_steps() {
        let mut process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        process.add_step(ReasoningStep::new(
            0,
            "x".to_string(),
            "analysis".to_string(),
        ));
        assert_eq!(process.step_count(), 1);
    }
}
