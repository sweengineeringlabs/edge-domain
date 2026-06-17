use serde::{Deserialize, Serialize};

/// Errors that can occur during prompt management and rendering
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PromptError {
    /// Template syntax is invalid
    #[serde(rename = "invalid_syntax")]
    InvalidSyntax {
        /// Description of the syntax error
        message: String,
    },

    /// Required variable is missing
    #[serde(rename = "missing_variable")]
    MissingVariable {
        /// Name of the missing variable
        variable_name: String,
    },

    /// Variable type mismatch
    #[serde(rename = "type_mismatch")]
    TypeMismatch {
        /// Variable name
        variable_name: String,
        /// Expected type
        expected: String,
        /// Actual type provided
        actual: String,
    },

    /// Variable value is invalid
    #[serde(rename = "invalid_value")]
    InvalidValue {
        /// Variable name
        variable_name: String,
        /// Reason for invalidity
        reason: String,
    },

    /// Context is incomplete (missing required variables)
    #[serde(rename = "incomplete_context")]
    IncompleteContext {
        /// List of missing variable names
        missing_variables: Vec<String>,
    },

    /// Rendering failed
    #[serde(rename = "render_failed")]
    RenderFailed(String),

    /// Cache error
    #[serde(rename = "cache_error")]
    CacheError(String),

    /// Token counting failed
    #[serde(rename = "tokenization_error")]
    TokenizationError(String),

    /// Unknown/unclassified error
    #[serde(rename = "unknown")]
    Unknown(String),
}

impl PromptError {
    /// Check if this error is recoverable (can retry)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            PromptError::CacheError(_) | PromptError::RenderFailed(_)
        )
    }

    /// Get human-readable error message
    pub fn message(&self) -> String {
        match self {
            PromptError::InvalidSyntax { message } => {
                format!("Invalid template syntax: {}", message)
            }
            PromptError::MissingVariable { variable_name } => {
                format!("Missing required variable: {}", variable_name)
            }
            PromptError::TypeMismatch {
                variable_name,
                expected,
                actual,
            } => {
                format!(
                    "Type mismatch for '{}': expected {}, got {}",
                    variable_name, expected, actual
                )
            }
            PromptError::InvalidValue {
                variable_name,
                reason,
            } => {
                format!("Invalid value for '{}': {}", variable_name, reason)
            }
            PromptError::IncompleteContext { missing_variables } => {
                format!(
                    "Incomplete context: missing {}",
                    missing_variables.join(", ")
                )
            }
            PromptError::RenderFailed(msg) => format!("Rendering failed: {}", msg),
            PromptError::CacheError(msg) => format!("Cache error: {}", msg),
            PromptError::TokenizationError(msg) => format!("Tokenization error: {}", msg),
            PromptError::Unknown(msg) => format!("Unknown error: {}", msg),
        }
    }
}
