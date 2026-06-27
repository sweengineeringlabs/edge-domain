//! [`SecurityContextBuilder`] — fluent builder for [`SecurityContext`].

use std::collections::HashMap;

use crate::Principal;

/// Fluent builder for constructing a [`SecurityContext`].
///
/// Prefer this over direct struct construction for contexts with multiple
/// optional fields. All methods and trait impls are defined in
/// `core::security::security_context_builder`.
pub struct SecurityContextBuilder {
    pub(crate) principal: Option<Box<dyn Principal>>,
    pub(crate) tenant_id: Option<String>,
    pub(crate) claims: HashMap<String, String>,
    pub(crate) trace_id: Option<String>,
    pub(crate) authenticated: bool,
    pub(crate) token: Option<String>,
    pub(crate) metadata: HashMap<String, String>,
    pub(crate) is_authorized: bool,
    pub(crate) extensions: std::collections::HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}
