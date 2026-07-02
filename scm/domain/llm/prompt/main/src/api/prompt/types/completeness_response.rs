//! [`CompletenessResponse`] — response for [`ContextManager::is_complete`](crate::api::prompt::traits::ContextManager::is_complete).

/// Whether all required (non-default) variables are satisfied.
#[derive(Debug, PartialEq)]
pub struct CompletenessResponse {
    /// `true` if every required variable has a value or default.
    pub complete: bool,
}
