//! No-op Validator implementation for testing the contract.

use crate::api::Validator;

/// A no-op validator that accepts all inputs.
/// Used for testing the contract; real implementations live in plugins.
pub(crate) struct NoopValidator;

impl Validator for NoopValidator {
    fn validate_agent_id(&self, _agent_id: &str) -> Result<(), String> {
        Ok(())
    }

    fn validate_skill_name(&self, _skill_name: &str) -> Result<(), String> {
        Ok(())
    }

    fn validate_skill_input(&self, _input: &str) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_validator_happy_validate_agent_id_accepts_valid_id() {
        assert!(NoopValidator.validate_agent_id("chief_engineer").is_ok());
    }

    #[test]
    fn test_noop_validator_happy_validate_skill_name_accepts_valid_name() {
        assert!(NoopValidator.validate_skill_name("code_review").is_ok());
    }

    #[test]
    fn test_noop_validator_happy_validate_skill_input_accepts_valid_json() {
        assert!(NoopValidator.validate_skill_input(r#"{"key":"value"}"#).is_ok());
    }

    #[test]
    fn test_noop_validator_edge_validate_agent_id_accepts_empty_string() {
        assert!(NoopValidator.validate_agent_id("").is_ok());
    }

    #[test]
    fn test_noop_validator_edge_validate_skill_name_accepts_special_chars() {
        assert!(NoopValidator.validate_skill_name("skill-with-dashes_and_underscores").is_ok());
    }

    #[test]
    fn test_noop_validator_edge_validate_skill_input_accepts_empty_string() {
        assert!(NoopValidator.validate_skill_input("").is_ok());
    }
}
