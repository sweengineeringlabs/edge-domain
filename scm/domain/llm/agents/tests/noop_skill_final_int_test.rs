#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Tests for the no-op `Skill` constructed via `saf::noop_skill`.

use edge_llm_agent::{
    NoopSkill, Skill, SkillDescriptionRequest, SkillNameRequest, SkillParametersRequest,
};

#[test]
fn test_noop_skill_name_is_noop() {
    assert_eq!(NoopSkill.name(SkillNameRequest).unwrap().name, "noop");
}

#[test]
fn test_noop_skill_description_is_non_empty() {
    assert!(!NoopSkill
        .description(SkillDescriptionRequest)
        .unwrap()
        .description
        .is_empty());
}

#[test]
fn test_noop_skill_has_no_parameters_by_default() {
    assert!(NoopSkill
        .parameters(SkillParametersRequest)
        .unwrap()
        .parameters
        .is_empty());
}
