//! [`ContextBuildRequest`] — request for [`ContextManager::build_context`](crate::api::prompt::traits::ContextManager::build_context).

/// Request to build a render context from registered variables. Carries no data.
#[derive(Debug, PartialEq)]
pub struct ContextBuildRequest;
