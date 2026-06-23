//! No-op [`Validator`] implementation for testing the contract.

use crate::api::NoopValidator;
use crate::api::Validator;

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
        assert_eq!(NoopValidator.validate_agent_id("chief_engineer"), Ok(()));
    }

    #[test]
    fn test_noop_validator_happy_validate_skill_name_accepts_valid_name() {
        assert_eq!(NoopValidator.validate_skill_name("code_review"), Ok(()));
    }

    #[test]
    fn test_noop_validator_happy_validate_skill_input_accepts_valid_json() {
        assert_eq!(
            NoopValidator.validate_skill_input(r#"{"key":"value"}"#),
            Ok(())
        );
    }

    #[test]
    fn test_noop_validator_edge_validate_agent_id_accepts_empty_string() {
        assert_eq!(NoopValidator.validate_agent_id(""), Ok(()));
    }

    #[test]
    fn test_noop_validator_edge_validate_skill_name_accepts_special_chars() {
        assert_eq!(
            NoopValidator.validate_skill_name("skill-with-dashes_and_underscores"),
            Ok(())
        );
    }

    #[test]
    fn test_noop_validator_edge_validate_skill_input_accepts_empty_string() {
        assert_eq!(NoopValidator.validate_skill_input(""), Ok(()));
    }
}
