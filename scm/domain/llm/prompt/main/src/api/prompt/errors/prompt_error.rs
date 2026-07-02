use serde::{Deserialize, Serialize};

/// Errors that can occur during prompt management and rendering
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
