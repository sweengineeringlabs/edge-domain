//! Integration tests for Validator trait contract.

use edge_llm_agent::Validator;

/// A strict validator implementation for testing error scenarios.
struct StrictValidator;

impl Validator for StrictValidator {
    fn validate_agent_id(&self, agent_id: &str) -> Result<(), String> {
        if agent_id.is_empty() {
            return Err("Agent ID cannot be empty".to_string());
        }
        if !agent_id.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err("Agent ID must contain only alphanumeric characters and underscores".to_string());
        }
        Ok(())
    }

    fn validate_skill_name(&self, skill_name: &str) -> Result<(), String> {
        if skill_name.is_empty() {
            return Err("Skill name cannot be empty".to_string());
        }
        if skill_name.len() > 100 {
            return Err("Skill name must not exceed 100 characters".to_string());
        }
        Ok(())
    }

    fn validate_skill_input(&self, input: &str) -> Result<(), String> {
        if input.is_empty() {
            return Err("Skill input cannot be empty".to_string());
        }
        // Basic JSON validation
        if !input.trim().starts_with('{') && !input.trim().starts_with('[') {
            return Err("Skill input must be valid JSON".to_string());
        }
        Ok(())
    }
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_alphanumeric_happy() {
    let validator = StrictValidator;
    assert!(validator.validate_agent_id("agent_123").is_ok());
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_with_underscores_happy() {
    let validator = StrictValidator;
    assert!(validator.validate_agent_id("chief_engineer").is_ok());
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_valid_name_happy() {
    let validator = StrictValidator;
    assert!(validator.validate_skill_name("code_review").is_ok());
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_json_object_happy() {
    let validator = StrictValidator;
    assert!(validator
        .validate_skill_input(r#"{"key":"value","nested":{"inner":42}}"#)
        .is_ok());
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_json_array_happy() {
    let validator = StrictValidator;
    assert!(validator.validate_skill_input(r#"[1,2,3,4,5]"#).is_ok());
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_empty_string_error() {
    let validator = StrictValidator;
    let result = validator.validate_agent_id("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_special_characters_error() {
    let validator = StrictValidator;
    let result = validator.validate_agent_id("agent-with-dashes");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("alphanumeric"));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_spaces_error() {
    let validator = StrictValidator;
    let result = validator.validate_agent_id("agent with spaces");
    assert!(result.is_err());
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_empty_string_error() {
    let validator = StrictValidator;
    let result = validator.validate_skill_name("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_exceeds_length_limit_error() {
    let validator = StrictValidator;
    let long_name = "a".repeat(101);
    let result = validator.validate_skill_name(&long_name);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("100 characters"));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_empty_string_error() {
    let validator = StrictValidator;
    let result = validator.validate_skill_input("");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("cannot be empty"));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_invalid_json_error() {
    let validator = StrictValidator;
    let result = validator.validate_skill_input("not json");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("JSON"));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_single_character_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_agent_id("a").is_ok());
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_all_underscores_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_agent_id("___").is_ok());
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_all_digits_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_agent_id("12345").is_ok());
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_exactly_100_chars_edge() {
    let validator = StrictValidator;
    let exact_name = "a".repeat(100);
    assert!(validator.validate_skill_name(&exact_name).is_ok());
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_single_character_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_skill_name("x").is_ok());
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_whitespace_before_json_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_skill_input("  {\"key\":\"value\"}").is_ok());
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_empty_json_object_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_skill_input("{}").is_ok());
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_empty_json_array_edge() {
    let validator = StrictValidator;
    assert!(validator.validate_skill_input("[]").is_ok());
}
