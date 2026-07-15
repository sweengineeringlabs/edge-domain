//! [`OutboundLenRequest`] — zero-sized marker for querying registry length.

/// Request to query the number of registered handles.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct OutboundLenRequest;
