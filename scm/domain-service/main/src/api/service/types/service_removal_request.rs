//! [`ServiceRemovalRequest`] — request to deregister a service.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceRemovalRequest {
    pub name: String,
}
