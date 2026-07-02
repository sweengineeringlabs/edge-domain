//! Constructors and builder methods for [`ThinkingProcessBuilder`].

use crate::api::{ReasoningStep, ThinkingProcess, ThinkingProcessBuilder};

impl ThinkingProcessBuilder {
    /// Start a new builder for the process with the given `id`.
    pub fn new(id: String) -> Self {
        Self {
            id,
            problem: String::new(),
            steps: vec![],
            is_complete: false,
            conclusion: None,
        }
    }

    /// Set the problem statement.
    pub fn problem(mut self, value: String) -> Self {
        self.problem = value;
        self
    }

    /// Append a reasoning step.
    pub fn step(mut self, step: ReasoningStep) -> Self {
        self.steps.push(step);
        self
    }

    /// Mark the process complete with a conclusion.
    pub fn conclusion(mut self, value: String) -> Self {
        self.is_complete = true;
        self.conclusion = Some(value);
        self
    }

    /// Build the [`ThinkingProcess`], computing the token total from steps.
    pub fn build(self) -> ThinkingProcess {
        let mut process = ThinkingProcess::new(self.id, self.problem);
        Self::apply_steps(&mut process, self.steps);
        process.is_complete = self.is_complete;
        process.conclusion = self.conclusion;
        process
    }

    /// Add each buffered step to `process` in order, accumulating its token total —
    /// extracted from [`Self::build`].
    fn apply_steps(process: &mut ThinkingProcess, steps: Vec<ReasoningStep>) {
        for step in steps {
            process.add_step(step);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_starts_with_empty_problem() {
        let builder = ThinkingProcessBuilder::new("p1".to_string());
        assert_eq!(builder.problem, "");
    }

    /// @covers: problem
    #[test]
    fn test_problem_sets_value() {
        let builder = ThinkingProcessBuilder::new("p1".to_string()).problem("q".to_string());
        assert_eq!(builder.problem, "q");
    }

    /// @covers: step
    #[test]
    fn test_step_appends_to_list() {
        let builder = ThinkingProcessBuilder::new("p1".to_string()).step(ReasoningStep::new(
            0,
            "x".to_string(),
            "analysis".to_string(),
        ));
        assert_eq!(builder.steps.len(), 1);
    }

    /// @covers: build
    #[test]
    fn test_build_produces_thinking_process() {
        let process = ThinkingProcessBuilder::new("p1".to_string())
            .conclusion("done".to_string())
            .build();
        assert_eq!(process.conclusion, Some("done".to_string()));
    }

    /// @covers: apply_steps
    #[test]
    fn test_apply_steps_adds_each_step_and_accumulates_tokens() {
        let mut process = ThinkingProcess::new("p1".to_string(), "q".to_string());
        let steps = vec![
            ReasoningStep::new(0, "x".to_string(), "analysis".to_string()).with_tokens(5),
            ReasoningStep::new(1, "y".to_string(), "analysis".to_string()).with_tokens(3),
        ];

        ThinkingProcessBuilder::apply_steps(&mut process, steps);

        assert_eq!(process.step_count(), 2);
        assert_eq!(process.total_tokens, 8);
    }

    /// @covers: conclusion
    #[test]
    fn test_conclusion_marks_builder_complete() {
        let builder = ThinkingProcessBuilder::new("p1".to_string()).conclusion("done".to_string());
        assert!(builder.is_complete);
    }
}
