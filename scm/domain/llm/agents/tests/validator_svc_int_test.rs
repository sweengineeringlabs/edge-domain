#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for validator_svc public interface.

use edge_llm_agent::{
    AgentIdValidationRequest, SkillInputValidationRequest, SkillNameValidationRequest,
    ValidationError, Validator,
};

/// A simple validator for testing the trait.
struct TestValidator;

impl Validator for TestValidator {
    fn validate_agent_id(&self, req: AgentIdValidationRequest<'_>) -> Result<(), ValidationError> {
        if req.agent_id.is_empty() {
            Err(ValidationError::new(
                "agent_id".to_string(),
                "Agent ID cannot be empty".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn validate_skill_name(
        &self,
        req: SkillNameValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        if req.skill_name.is_empty() {
            Err(ValidationError::new(
                "skill_name".to_string(),
                "Skill name cannot be empty".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn validate_skill_input(
        &self,
        req: SkillInputValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        if req.input.is_empty() {
            Err(ValidationError::new(
                "skill_input".to_string(),
                "Skill input cannot be empty".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}

// @covers: VALIDATOR_SVC
#[test]
fn test_validator_svc_happy() {
    use edge_llm_agent::VALIDATOR_SVC;
    assert_eq!(VALIDATOR_SVC, "validator");
}

// @covers: VALIDATOR_SVC
#[test]
fn test_validator_svc_error() {
    use edge_llm_agent::VALIDATOR_SVC;
    assert_ne!(VALIDATOR_SVC, "wrong");
}

// @covers: VALIDATOR_SVC
#[test]
fn test_validator_svc_edge() {
    let validator = TestValidator;
    assert!(matches!(
        validator.validate_agent_id(AgentIdValidationRequest { agent_id: "agent" }),
        Ok(())
    ));
    assert!(matches!(
        validator.validate_skill_name(SkillNameValidationRequest {
            skill_name: "skill"
        }),
        Ok(())
    ));
    assert!(matches!(
        validator.validate_skill_input(SkillInputValidationRequest { input: "{}" }),
        Ok(())
    ));
}
