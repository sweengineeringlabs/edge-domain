//! [`ApplicationRunRequest`] — zero-sized marker for booting an `Application`.

/// Request to boot the application.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ApplicationRunRequest;
