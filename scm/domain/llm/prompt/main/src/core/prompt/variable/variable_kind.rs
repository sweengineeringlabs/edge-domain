//! Inherent methods for [`VariableKind`].

use crate::api::VariableKind;

impl VariableKind {
    /// Check if this type can be directly serialized to string
    pub fn is_scalar(&self) -> bool {
        Self::scalar_variants().contains(self)
    }

    /// The variants considered scalar (directly serializable to a plain string).
    fn scalar_variants() -> &'static [VariableKind] {
        &[
            VariableKind::String,
            VariableKind::Number,
            VariableKind::Boolean,
        ]
    }

    /// Get human-readable type name
    pub fn as_str(&self) -> &'static str {
        match self {
            VariableKind::String => "string",
            VariableKind::Number => "number",
            VariableKind::Boolean => "boolean",
            VariableKind::List => "list",
            VariableKind::Object => "object",
            VariableKind::Json => "json",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: is_scalar
    #[test]
    fn test_is_scalar_string_is_true() {
        assert!(VariableKind::String.is_scalar());
    }

    /// @covers: is_scalar
    #[test]
    fn test_is_scalar_list_is_false() {
        assert!(!VariableKind::List.is_scalar());
    }

    /// @covers: as_str
    #[test]
    fn test_as_str_returns_expected_literal() {
        assert_eq!(VariableKind::Json.as_str(), "json");
    }

    /// @covers: scalar_variants
    #[test]
    fn test_scalar_variants_excludes_list() {
        assert!(!VariableKind::scalar_variants().contains(&VariableKind::List));
        assert!(VariableKind::scalar_variants().contains(&VariableKind::String));
    }
}
