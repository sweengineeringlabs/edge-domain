//! Constructor for [`SkillMetadata`].

use crate::api::{SkillMetadata, SkillMetadataBuilder};

impl SkillMetadata {
    /// Create a new SkillMetadataBuilder for constructing SkillMetadata.
    pub fn builder() -> SkillMetadataBuilder {
        SkillMetadataBuilder::new()
    }
}

impl Default for SkillMetadata {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: builder
    #[test]
    fn test_builder_returns_empty_builder() {
        let metadata = SkillMetadata::builder().build();
        assert_eq!(metadata.name, "");
    }

    /// @covers: default
    #[test]
    fn test_default_has_empty_name() {
        assert_eq!(SkillMetadata::default().name, "");
    }
}
