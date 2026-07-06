#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Integration tests for Validator trait contract.

use edge_llm_agent::{
    AgentIdValidationRequest, SkillInputValidationRequest, SkillNameValidationRequest,
    ValidationError, Validator,
};

/// A strict validator implementation for testing error scenarios.
struct StrictValidator;

impl Validator for StrictValidator {
    fn validate_agent_id(&self, req: AgentIdValidationRequest<'_>) -> Result<(), ValidationError> {
        if req.agent_id.is_empty() {
            return Err(ValidationError::new(
                "agent_id".to_string(),
                "Agent ID cannot be empty".to_string(),
            ));
        }
        if !req
            .agent_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_')
        {
            return Err(ValidationError::new(
                "agent_id".to_string(),
                "Agent ID must contain only alphanumeric characters and underscores".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_skill_name(
        &self,
        req: SkillNameValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        if req.skill_name.is_empty() {
            return Err(ValidationError::new(
                "skill_name".to_string(),
                "Skill name cannot be empty".to_string(),
            ));
        }
        if req.skill_name.len() > 100 {
            return Err(ValidationError::new(
                "skill_name".to_string(),
                "Skill name must not exceed 100 characters".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_skill_input(
        &self,
        req: SkillInputValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        if req.input.is_empty() {
            return Err(ValidationError::new(
                "skill_input".to_string(),
                "Skill input cannot be empty".to_string(),
            ));
        }
        // Basic JSON validation
        if !req.input.trim().starts_with('{') && !req.input.trim().starts_with('[') {
            return Err(ValidationError::new(
                "skill_input".to_string(),
                "Skill input must be valid JSON".to_string(),
            ));
        }
        Ok(())
    }
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_alphanumeric_happy() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_agent_id(AgentIdValidationRequest {
            agent_id: "agent_123"
        }),
        Ok(())
    ));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_with_underscores_happy() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_agent_id(AgentIdValidationRequest {
            agent_id: "chief_engineer"
        }),
        Ok(())
    ));
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_valid_name_happy() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_name(SkillNameValidationRequest {
            skill_name: "code_review"
        }),
        Ok(())
    ));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_json_object_happy() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_input(SkillInputValidationRequest {
            input: r#"{"key":"value","nested":{"inner":42}}"#
        }),
        Ok(())
    ));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_json_array_happy() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_input(SkillInputValidationRequest {
            input: r#"[1,2,3,4,5]"#
        }),
        Ok(())
    ));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_empty_string_error() {
    let validator = StrictValidator;
    let result = validator.validate_agent_id(AgentIdValidationRequest { agent_id: "" });
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("cannot be empty"));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_special_characters_error() {
    let validator = StrictValidator;
    let result = validator.validate_agent_id(AgentIdValidationRequest {
        agent_id: "agent-with-dashes",
    });
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("alphanumeric"));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_spaces_error() {
    let validator = StrictValidator;
    let result = validator.validate_agent_id(AgentIdValidationRequest {
        agent_id: "agent with spaces",
    });
    assert!(result.is_err());
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_empty_string_error() {
    let validator = StrictValidator;
    let result = validator.validate_skill_name(SkillNameValidationRequest { skill_name: "" });
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("cannot be empty"));
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_exceeds_length_limit_error() {
    let validator = StrictValidator;
    let long_name = "a".repeat(101);
    let result = validator.validate_skill_name(SkillNameValidationRequest {
        skill_name: &long_name,
    });
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("100 characters"));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_empty_string_error() {
    let validator = StrictValidator;
    let result = validator.validate_skill_input(SkillInputValidationRequest { input: "" });
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("cannot be empty"));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_invalid_json_error() {
    let validator = StrictValidator;
    let result = validator.validate_skill_input(SkillInputValidationRequest { input: "not json" });
    assert!(result.is_err());
    assert!(result.unwrap_err().reason.contains("JSON"));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_single_character_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_agent_id(AgentIdValidationRequest { agent_id: "a" }),
        Ok(())
    ));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_all_underscores_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_agent_id(AgentIdValidationRequest { agent_id: "___" }),
        Ok(())
    ));
}

// @covers: validate_agent_id
#[test]
fn test_validate_agent_id_all_digits_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_agent_id(AgentIdValidationRequest { agent_id: "12345" }),
        Ok(())
    ));
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_exactly_100_chars_edge() {
    let validator = StrictValidator;
    let exact_name = "a".repeat(100);
    assert!(matches!(
        validator.validate_skill_name(SkillNameValidationRequest {
            skill_name: &exact_name
        }),
        Ok(())
    ));
}

// @covers: validate_skill_name
#[test]
fn test_validate_skill_name_single_character_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_name(SkillNameValidationRequest { skill_name: "x" }),
        Ok(())
    ));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_whitespace_before_json_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_input(SkillInputValidationRequest {
            input: "  {\"key\":\"value\"}"
        }),
        Ok(())
    ));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_empty_json_object_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_input(SkillInputValidationRequest { input: "{}" }),
        Ok(())
    ));
}

// @covers: validate_skill_input
#[test]
fn test_validate_skill_input_empty_json_array_edge() {
    let validator = StrictValidator;
    assert!(matches!(
        validator.validate_skill_input(SkillInputValidationRequest { input: "[]" }),
        Ok(())
    ));
}
