use crate::api::types::Role;

/// Response for [`Agent::supported_role`](crate::api::traits::Agent::supported_role).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportedRoleResponse {
    /// The conversation role this agent speaks as.
    pub role: Role,
}
