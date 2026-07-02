#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `Validator` constructed via `saf::noop_validator`.

use edge_llm_agent::{
    AgentIdValidationRequest, NoopValidator, SkillInputValidationRequest,
    SkillNameValidationRequest, Validator,
};

#[test]
fn test_noop_validator_accepts_agent_id() {
    assert!(matches!(
        NoopValidator.validate_agent_id(AgentIdValidationRequest { agent_id: "agent" }),
        Ok(())
    ));
}

#[test]
fn test_noop_validator_accepts_skill_name() {
    assert!(matches!(
        NoopValidator.validate_skill_name(SkillNameValidationRequest {
            skill_name: "skill"
        }),
        Ok(())
    ));
}

#[test]
fn test_noop_validator_accepts_empty_input() {
    assert!(matches!(
        NoopValidator.validate_skill_input(SkillInputValidationRequest { input: "" }),
        Ok(())
    ));
}
