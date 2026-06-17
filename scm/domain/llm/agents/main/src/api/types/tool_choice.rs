use serde::{Deserialize, Serialize};

/// Controls which tool (if any) agent must call
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToolChoice {
    /// Agent decides whether to call a tool
    Auto,

    /// Agent must not call any tool
    None,

    /// Agent must call at least one tool
    Required,

    /// Agent must call the named tool
    Function {
        /// The name of the tool the agent must call.
        name: String,
    },
}
