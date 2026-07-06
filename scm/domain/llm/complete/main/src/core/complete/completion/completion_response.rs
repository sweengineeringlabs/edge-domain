//! Constructor for [`CompletionResponse`].

use crate::api::{CompletionResponse, FinishReason};

impl CompletionResponse {
    /// Construct a response with text content.
    pub fn text(
        id: impl Into<String>,
        model: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            model: model.into(),
            content: Some(content.into()),
            finish_reason: Self::default_finish_reason(),
            ..Default::default()
        }
    }

    /// Finish reason assigned to a plain-text response.
    fn default_finish_reason() -> FinishReason {
        FinishReason::Stop
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: text
    #[test]
    fn test_text_sets_content_and_finish_reason() {
        let resp = CompletionResponse::text("id1", "gpt-4", "hi");
        assert_eq!(resp.content, Some("hi".to_string()));
        assert_eq!(resp.finish_reason, FinishReason::Stop);
    }

    /// @covers: default_finish_reason
    #[test]
    fn test_default_finish_reason_is_stop() {
        assert_eq!(
            CompletionResponse::default_finish_reason(),
            FinishReason::Stop
        );
    }
}
