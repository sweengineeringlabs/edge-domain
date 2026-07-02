//! Constructor for [`AgentMetadata`].

use crate::api::{AgentMetadata, AgentMetadataBuilder};

impl AgentMetadata {
    /// Create a new AgentMetadataBuilder for constructing AgentMetadata.
    pub fn builder() -> AgentMetadataBuilder {
        AgentMetadataBuilder::new()
    }
}

impl Default for AgentMetadata {
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
        let metadata = AgentMetadata::builder().build();
        assert_eq!(metadata.id, "");
    }

    /// @covers: default
    #[test]
    fn test_default_has_empty_id() {
        assert_eq!(AgentMetadata::default().id, "");
    }
}
