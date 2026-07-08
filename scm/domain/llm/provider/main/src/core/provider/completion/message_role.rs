//! Conversions for [`MessageRole`].

use edge_llm_complete::Role;

use crate::api::MessageRole;

impl MessageRole {
    /// Map onto the corresponding [`edge_llm_complete::Role`].
    pub(crate) fn into_role(self) -> Role {
        match self {
            Self::User => Role::User,
            Self::Assistant => Role::Assistant,
            Self::Tool => Role::Tool,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: into_role
    #[test]
    fn test_into_role_maps_user() {
        assert_eq!(MessageRole::User.into_role(), Role::User);
    }

    /// @covers: into_role
    #[test]
    fn test_into_role_maps_assistant() {
        assert_eq!(MessageRole::Assistant.into_role(), Role::Assistant);
    }

    /// @covers: into_role
    #[test]
    fn test_into_role_maps_tool() {
        assert_eq!(MessageRole::Tool.into_role(), Role::Tool);
    }
}
