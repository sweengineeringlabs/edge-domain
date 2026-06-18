use serde::{Deserialize, Serialize};

/// Reason a completion finished
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum FinishReason {
    /// Model reached natural stop
    #[serde(rename = "stop")]
    Stop,

    /// Max tokens reached
    #[serde(rename = "length")]
    Length,

    /// Tool call requested
    #[serde(rename = "tool_calls")]
    ToolCalls,

    /// Content filter triggered
    #[serde(rename = "content_filter")]
    ContentFilter,

    /// Unexpected end (error)
    #[serde(rename = "error")]
    Error,
}

#[cfg(test)]
mod tests {
    use super::FinishReason;

    #[test]
    fn test_finish_reason_variants_distinct() {
        assert_ne!(FinishReason::Stop, FinishReason::Length);
        assert_ne!(FinishReason::ToolCalls, FinishReason::Error);
    }

    #[test]
    fn test_finish_reason_equality() {
        assert_eq!(FinishReason::Stop, FinishReason::Stop);
    }

    #[test]
    fn test_finish_reason_serde_roundtrip() {
        let json = serde_json::to_string(&FinishReason::ContentFilter).expect("serialize");
        let back: FinishReason = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, FinishReason::ContentFilter);
    }
}
