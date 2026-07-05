//! [`LenRequest`] — zero-sized marker for querying registry length.

/// Request for the number of registered entries.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LenRequest;
