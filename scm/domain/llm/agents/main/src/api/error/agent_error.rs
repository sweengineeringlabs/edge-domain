//! Agent operation errors.

use thiserror::Error;

/// Errors that occur during agent operations.
#[derive(Debug, Error)]
pub enum AgentError {
    /// The requested agent could not be found.
    #[error("Agent '{0}' not found")]
    NotFound(String),

    /// The requested skill is not available on the agent.
    #[error("Skill '{0}' not available on agent")]
    SkillNotFound(String),

    /// The agent specification is invalid.
    #[error("Invalid agent specification: {0}")]
    InvalidSpec(String),

    /// Execution of a skill failed.
    #[error("Skill execution failed: {0}")]
    ExecutionFailed(String),

    /// The agent is not in a valid state for the requested operation.
    #[error("Agent is not in a valid state for this operation: {0}")]
    InvalidState(String),
}
