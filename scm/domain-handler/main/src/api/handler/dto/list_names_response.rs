//! [`ListNamesResponse`] — response for [`ServiceRegistry::list_names`](crate::api::handler::traits::ServiceRegistry::list_names).

/// All registered service names.
pub struct ListNamesResponse {
    /// The names of all registered services.
    pub names: Vec<String>,
}
