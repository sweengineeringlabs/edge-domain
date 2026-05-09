//! Outbound error formatting adapter.

use crate::api::handler_error::HandlerError;

/// Wrap a handler error as a human-readable string for inbound transport layers.
pub fn format_error(err: &HandlerError) -> String {
    format!("handler error: {err}")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: format_error
    #[test]
    fn test_format_error_produces_non_empty_string() {
        let err = HandlerError::Other("boom".into());
        let msg = format_error(&err);
        assert!(!msg.is_empty());
        assert!(msg.contains("handler error"));
    }
}
