//! Agent operation errors.

use thiserror::Error;

/// Errors that occur during agent operations.
#[derive(Debug, Error)]
pub enum AgentError {
    #[error("Agent '{0}' not found")]
    NotFound(String),

    #[error("Skill '{0}' not available on agent")]
    SkillNotFound(String),

    #[error("Invalid agent specification: {0}")]
    InvalidSpec(String),

    #[error("Skill execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Agent is not in a valid state for this operation: {0}")]
    InvalidState(String),
}
