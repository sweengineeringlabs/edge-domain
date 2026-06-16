//! Comprehensive tests for Validator trait.

use edge_llm_agent::Validator;

struct TestValidator {
    should_fail: bool,
}

impl Validator for TestValidator {
    fn validate_agent_id(&self, agent_id: &str) -> Result<(), String> {
        if self.should_fail {
            Err("Validation failed".to_string())
        } else if agent_id.is_empty() {
            Err("Empty agent ID".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_skill_name(&self, skill_name: &str) -> Result<(), String> {
        if self.should_fail {
            Err("Validation failed".to_string())
        } else if skill_name.is_empty() {
            Err("Empty skill name".to_string())
        } else {
            Ok(())
        }
    }

    fn validate_skill_input(&self, input: &str) -> Result<(), String> {
        if self.should_fail {
            Err("Validation failed".to_string())
        } else if input.is_empty() {
            Err("Empty input".to_string())
        } else {
            Ok(())
        }
    }
}

/// @covers Validator::validate_agent_id happy path
#[test]
fn test_validate_agent_id_happy_valid_id() {
    let validator = TestValidator { should_fail: false };
    let result = validator.validate_agent_id("test_agent");
    assert!(result.is_ok());
}

/// @covers Validator::validate_agent_id error path
#[test]
fn test_validate_agent_id_error_validation_fails() {
    let validator = TestValidator { should_fail: true };
    let result = validator.validate_agent_id("any_id");
    assert!(result.is_err());
}

/// @covers Validator::validate_agent_id edge case empty
#[test]
fn test_validate_agent_id_edge_empty_id() {
    let validator = TestValidator { should_fail: false };
    let result = validator.validate_agent_id("");
    assert!(result.is_err());
}

/// @covers Validator::validate_skill_name happy path
#[test]
fn test_validate_skill_name_happy_valid_name() {
    let validator = TestValidator { should_fail: false };
    let result = validator.validate_skill_name("test_skill");
    assert!(result.is_ok());
}

/// @covers Validator::validate_skill_name error path
#[test]
fn test_validate_skill_name_error_validation_fails() {
    let validator = TestValidator { should_fail: true };
    let result = validator.validate_skill_name("any_name");
    assert!(result.is_err());
}

/// @covers Validator::validate_skill_name edge case empty
#[test]
fn test_validate_skill_name_edge_empty_name() {
    let validator = TestValidator { should_fail: false };
    let result = validator.validate_skill_name("");
    assert!(result.is_err());
}

/// @covers Validator::validate_skill_input happy path
#[test]
fn test_validate_skill_input_happy_valid_input() {
    let validator = TestValidator { should_fail: false };
    let result = validator.validate_skill_input(r#"{"key":"value"}"#);
    assert!(result.is_ok());
}

/// @covers Validator::validate_skill_input error path
#[test]
fn test_validate_skill_input_error_validation_fails() {
    let validator = TestValidator { should_fail: true };
    let result = validator.validate_skill_input(r#"{"key":"value"}"#);
    assert!(result.is_err());
}

/// @covers Validator::validate_skill_input edge case empty
#[test]
fn test_validate_skill_input_edge_empty_input() {
    let validator = TestValidator { should_fail: false };
    let result = validator.validate_skill_input("");
    assert!(result.is_err());
}
