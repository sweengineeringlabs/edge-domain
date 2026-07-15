//! `NoopService` — a no-operation service implementation.

/// A no-operation [`Service`](crate::api::service::traits::service::Service) that accepts
/// `()` requests and immediately returns `()`.
///
/// Useful as a sentinel or placeholder where a real service implementation is not required.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopService;
