//! [`CompletenessRequest`] — request for [`ContextManager::is_complete`](crate::api::prompt::traits::ContextManager::is_complete).

/// Request to check whether all required variables are satisfied. Carries no data.
#[derive(Debug, PartialEq)]
pub struct CompletenessRequest;
