//! Constructor for [`ParameterDocumentation`].

use crate::api::ParameterDocumentation;

impl ParameterDocumentation {
    /// Creates a new parameter documentation entry with the given core fields.
    pub fn new(name: String, description: String, param_type: String, required: bool) -> Self {
        Self {
            name,
            description,
            param_type,
            required,
            default: None,
            examples: Vec::new(),
            validation_rules: None,
        }
    }
}

impl Default for ParameterDocumentation {
    fn default() -> Self {
        Self::new(String::new(), String::new(), String::new(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: default
    #[test]
    fn test_default_is_not_required() {
        assert!(!ParameterDocumentation::default().required);
    }

    /// @covers: new
    #[test]
    fn test_new_sets_core_fields() {
        let doc = ParameterDocumentation::new(
            "name".to_string(),
            "desc".to_string(),
            "string".to_string(),
            true,
        );
        assert_eq!(doc.name, "name");
        assert!(doc.required);
    }
}
