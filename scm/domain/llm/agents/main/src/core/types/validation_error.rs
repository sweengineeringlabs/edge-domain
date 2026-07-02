//! Constructor for [`ValidationError`].

use crate::api::ValidationError;

impl ValidationError {
    /// Creates a new validation error for the given field and reason.
    pub fn new(field: String, reason: String) -> Self {
        Self {
            field: Self::normalized(field),
            reason,
        }
    }

    /// Strip leading/trailing whitespace from a field name.
    fn normalized(field: String) -> String {
        field.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_field_and_reason() {
        let err = ValidationError::new("age".to_string(), "must be positive".to_string());
        assert_eq!(err.field, "age");
        assert_eq!(err.reason, "must be positive");
    }

    /// @covers: normalized
    #[test]
    fn test_normalized_strips_whitespace() {
        assert_eq!(ValidationError::normalized("  age  ".to_string()), "age");
    }
}
