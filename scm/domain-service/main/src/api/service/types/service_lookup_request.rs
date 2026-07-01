//! [`ServiceLookupRequest`] — request to retrieve a service.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceLookupRequest {
    pub name: String,
}
