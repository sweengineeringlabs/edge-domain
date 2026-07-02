//! [`RegisterVariableRequest`] — request for [`ContextManager::register_variable`](crate::api::prompt::traits::ContextManager::register_variable).

use crate::api::prompt::types::Variable;

/// Request to register `var` under `name`.
#[derive(Debug, PartialEq)]
pub struct RegisterVariableRequest<'a> {
    /// The name to register the variable under.
    pub name: String,
    /// The variable to register.
    pub var: &'a Variable,
}
