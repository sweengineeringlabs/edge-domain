use serde::{Deserialize, Serialize};

/// Incremental tool call in a streamed response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Tool call index
    pub index: usize,

    /// Tool call ID
    pub id: Option<String>,

    /// Tool name
    pub name: Option<String>,

    /// Partial arguments (JSON string fragment)
    pub arguments: Option<String>,
}

impl ToolCallDelta {
    /// Create a new tool call delta
    pub fn new(index: usize) -> Self {
        Self {
            index,
            id: None,
            name: None,
            arguments: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ToolCallDelta;

    #[test]
    fn test_new_sets_index() {
        let delta = ToolCallDelta::new(3);
        assert_eq!(delta.index, 3);
        assert!(delta.id.is_none());
        assert!(delta.name.is_none());
    }

    #[test]
    fn test_tool_call_delta_clone() {
        let delta = ToolCallDelta::new(1);
        assert_eq!(delta.clone().index, 1);
    }

    #[test]
    fn test_tool_call_delta_serde_roundtrip() {
        let delta = ToolCallDelta::new(2);
        let json = serde_json::to_string(&delta).expect("serialize");
        let back: ToolCallDelta = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.index, 2);
    }
}
