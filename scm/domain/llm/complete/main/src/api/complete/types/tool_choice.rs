use serde::{Deserialize, Serialize};

/// How the model should decide whether and which tool to call.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoice {
    /// Model decides automatically.
    #[default]
    Auto,
    /// Model must not call any tool.
    None,
    /// Model must call at least one tool.
    Required,
    /// Model must call the named function.
    Function {
        /// Name of the tool to force.
        name: String,
    },
}
