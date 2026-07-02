//! [`ClearVariablesRequest`] — request for [`ContextManager::clear`](crate::api::prompt::traits::ContextManager::clear).

/// Request to remove all registered variables. Carries no data.
#[derive(Debug, PartialEq)]
pub struct ClearVariablesRequest;
