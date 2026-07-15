//! [`ServiceRemovalResponse`] — wrapper for deregister result (was_present).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceRemovalResponse {
    pub was_present: bool,
}
