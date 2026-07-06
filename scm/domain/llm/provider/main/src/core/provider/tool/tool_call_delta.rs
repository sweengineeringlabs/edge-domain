//! Constructor for [`ToolCallDelta`].

use crate::api::ToolCallDelta;

impl ToolCallDelta {
    /// Create a new tool call delta
    pub fn new(index: usize) -> Self {
        Self {
            index,
            id: Self::unknown(),
            name: Self::unknown(),
            arguments: Self::unknown(),
        }
    }

    /// Placeholder for a field not yet observed in the stream.
    fn unknown() -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_sets_index_and_defaults_rest() {
        let delta = ToolCallDelta::new(3);
        assert_eq!(delta.index, 3);
        assert!(delta.id.is_none());
    }

    /// @covers: unknown
    #[test]
    fn test_unknown_is_none() {
        assert_eq!(ToolCallDelta::unknown(), None);
    }
}
