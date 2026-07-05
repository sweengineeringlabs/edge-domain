//! [`ValidationRequest`] — zero-sized marker for requesting a backend reachability check.

/// Request to check whether the observability backend is reachable.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ValidationRequest;
