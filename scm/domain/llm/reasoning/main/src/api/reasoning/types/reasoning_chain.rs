use crate::api::reasoning::types::ThinkingProcess;
use serde::{Deserialize, Serialize};

/// Chain of multiple reasoning processes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReasoningChain {
    /// Unique ID for this reasoning chain
    pub id: String,

    /// Ordered list of reasoning processes
    pub processes: Vec<ThinkingProcess>,

    /// Whether chain is complete
    pub is_complete: bool,

    /// Final answer after all processes
    pub final_answer: Option<String>,
}

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
        let total: f32 = self.processes.iter().map(|p| p.average_confidence()).sum();
        total / self.processes.len() as f32
    }

    /// Check if all processes are complete
    pub fn all_complete(&self) -> bool {
        self.processes.iter().all(|p| p.is_complete)
    }
}
