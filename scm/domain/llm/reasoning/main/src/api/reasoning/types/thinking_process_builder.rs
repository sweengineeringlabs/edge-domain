//! `ThinkingProcessBuilder` — fluent builder for [`ThinkingProcess`].

use crate::api::reasoning::types::{ReasoningStep, ThinkingProcess};

/// Fluent builder for [`ThinkingProcess`].
#[derive(Clone, Debug)]
pub struct ThinkingProcessBuilder {
    id: String,
    problem: String,
    steps: Vec<ReasoningStep>,
    is_complete: bool,
    conclusion: Option<String>,
}

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
        for step in self.steps {
            process.add_step(step);
        }
        process.is_complete = self.is_complete;
        process.conclusion = self.conclusion;
        process
    }
}
