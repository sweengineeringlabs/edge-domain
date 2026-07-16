//! [`NoopRequest`] — zero-sized payload for [`NoopService`](crate::api::NoopService).

/// The request payload accepted by [`NoopService`](crate::api::NoopService) — carries no data.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoopRequest;
