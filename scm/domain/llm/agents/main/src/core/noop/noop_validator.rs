//! No-op [`Validator`] implementation for testing the contract.

use crate::api::NoopValidator;
use crate::api::Validator;
use crate::api::{
    AgentIdValidationRequest, SkillInputValidationRequest, SkillNameValidationRequest,
    ValidationError,
};

impl Validator for NoopValidator {
    fn validate_agent_id(&self, _req: AgentIdValidationRequest<'_>) -> Result<(), ValidationError> {
        Ok(())
    }

    fn validate_skill_name(
        &self,
        _req: SkillNameValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }

    fn validate_skill_input(
        &self,
        _req: SkillInputValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_validator_happy_validate_agent_id_accepts_valid_id() {
        assert!(matches!(
            NoopValidator.validate_agent_id(AgentIdValidationRequest {
                agent_id: "chief_engineer"
            }),
            Ok(())
        ));
    }

    #[test]
    fn test_noop_validator_happy_validate_skill_name_accepts_valid_name() {
        assert!(matches!(
            NoopValidator.validate_skill_name(SkillNameValidationRequest {
                skill_name: "code_review"
            }),
            Ok(())
        ));
    }

    #[test]
    fn test_noop_validator_happy_validate_skill_input_accepts_valid_json() {
        assert!(matches!(
            NoopValidator.validate_skill_input(SkillInputValidationRequest {
                input: r#"{"key":"value"}"#
            }),
            Ok(())
        ));
    }

    #[test]
    fn test_noop_validator_edge_validate_agent_id_accepts_empty_string() {
        assert!(matches!(
            NoopValidator.validate_agent_id(AgentIdValidationRequest { agent_id: "" }),
            Ok(())
        ));
    }

    #[test]
    fn test_noop_validator_edge_validate_skill_name_accepts_special_chars() {
        assert!(matches!(
            NoopValidator.validate_skill_name(SkillNameValidationRequest {
                skill_name: "skill-with-dashes_and_underscores"
            }),
            Ok(())
        ));
    }

    #[test]
    fn test_noop_validator_edge_validate_skill_input_accepts_empty_string() {
        assert!(matches!(
            NoopValidator.validate_skill_input(SkillInputValidationRequest { input: "" }),
            Ok(())
        ));
    }
}
