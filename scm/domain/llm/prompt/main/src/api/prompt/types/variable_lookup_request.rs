//! [`VariableLookupRequest`] — request for [`ContextManager::get_variable`](crate::api::prompt::traits::ContextManager::get_variable).

/// Request to look up a registered variable by name.
#[derive(Debug, PartialEq)]
pub struct VariableLookupRequest<'a> {
    /// The name of the variable to look up.
    pub name: &'a str,
}
