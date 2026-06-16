//! Service exports for skill-related types and traits.

mod metadata;
mod skill;

pub use metadata::{SkillMetadata, SKILL_METADATA_SVC};
pub use skill::{Skill, SKILL_SVC};
