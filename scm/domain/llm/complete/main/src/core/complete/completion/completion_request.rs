//! Constructor for [`CompletionRequest`].

use crate::api::{CompletionRequest, Message};

impl CompletionRequest {
    /// Construct a minimal request with a model and message list.
    pub fn new(model: impl Into<String>, messages: Vec<Message>) -> Self {
        Self {
            model: Self::normalized_model(model.into()),
            messages,
            ..Default::default()
        }
    }

    /// Strip leading/trailing whitespace from a model id.
    fn normalized_model(model: String) -> String {
        model.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_model_and_messages() {
        let req = CompletionRequest::new("gpt-4", vec![Message::user("hi")]);
        assert_eq!(req.model, "gpt-4");
        assert_eq!(req.messages.len(), 1);
    }

    /// @covers: normalized_model
    #[test]
    fn test_normalized_model_strips_whitespace() {
        assert_eq!(
            CompletionRequest::normalized_model("  gpt-4  ".to_string()),
            "gpt-4"
        );
    }
}
