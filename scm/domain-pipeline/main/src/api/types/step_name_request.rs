//! [`StepNameRequest`] — zero-sized marker for [`Step::name`](crate::Step::name).

/// Marker request for querying a step's human-readable name.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StepNameRequest;
