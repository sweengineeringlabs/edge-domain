//! [`ServiceLookupRequest`] — request for [`ServiceRegistry::get`](crate::api::handler::traits::ServiceRegistry::get).

/// Request to look up a service by name.
pub struct ServiceLookupRequest {
    /// The name of the service to look up.
    pub name: String,
}
