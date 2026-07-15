//! [`RuntimeBootResponse`] — wrapper for a completed boot-gate run.

/// Result of [`AppRuntime::boot`](crate::api::AppRuntime::boot) completing successfully.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeBootResponse;
