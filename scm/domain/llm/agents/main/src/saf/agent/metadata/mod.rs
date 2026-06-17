//! SAF service exports for skill metadata types.

mod builder_svc;
mod svc;

pub use builder_svc::{SkillMetadataBuilder, SKILL_METADATA_BUILDER_SVC};
pub use svc::{SkillMetadata, SKILL_METADATA_SVC};
