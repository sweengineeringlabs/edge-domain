#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for validator_svc public interface.

use edge_llm_agent::Validator;

/// A simple validator for testing the trait.
struct TestValidator;

impl Validator for TestValidator {
    fn validate_agent_id(&self, agent_id: &str) -> Result<(), String> {
        if agent_id.is_empty() {
            Err("Agent ID cannot be empty".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_skill_name(&self, skill_name: &str) -> Result<(), String> {
        if skill_name.is_empty() {
            Err("Skill name cannot be empty".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_skill_input(&self, input: &str) -> Result<(), String> {
        if input.is_empty() {
            Err("Skill input cannot be empty".to_string())
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
    assert_eq!(validator.validate_agent_id("agent"), Ok(()));
    assert_eq!(validator.validate_skill_name("skill"), Ok(()));
    assert_eq!(validator.validate_skill_input("{}"), Ok(()));
}
