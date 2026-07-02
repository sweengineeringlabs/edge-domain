use serde::{Deserialize, Serialize};

/// LLM agent lifecycle state
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AgentState {
    /// Agent created but not yet started
    Idle,

    /// Agent actively reasoning/executing
    Running,

    /// Agent paused (waiting for input, checkpointing, etc.)
    Paused,

    /// Agent in reasoning phase (thinking deeply, exploring options)
    Thinking,

    /// Agent completed successfully
    Completed,
}
