//! [`VariableLookupResponse`] — response for [`ContextManager::get_variable`](crate::api::prompt::traits::ContextManager::get_variable).

use crate::api::prompt::types::Variable;

/// The variable found for the requested name, if any.
#[derive(Debug, PartialEq)]
pub struct VariableLookupResponse<'a> {
    /// The variable registered under the requested name, if present.
    pub variable: Option<&'a Variable>,
}
