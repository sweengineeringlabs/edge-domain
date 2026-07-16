//! [`NoopResponse`] — zero-sized payload produced by [`NoopService`](crate::api::NoopService).

/// The response payload produced by [`NoopService`](crate::api::NoopService) — carries no data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoopResponse;
