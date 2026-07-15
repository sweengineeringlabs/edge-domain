//! [`LogDrainBuildRequest`] — zero-sized marker for requesting a `LogDrain`.

/// Request to build a [`LogDrain`](crate::api::LogDrain).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LogDrainBuildRequest;
