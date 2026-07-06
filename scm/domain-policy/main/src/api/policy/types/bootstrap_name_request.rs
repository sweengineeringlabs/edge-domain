//! [`BootstrapNameRequest`] — zero-sized marker for querying a bootstrap implementation's name.

/// Request for a bootstrap implementation's stable identifier.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameRequest;
