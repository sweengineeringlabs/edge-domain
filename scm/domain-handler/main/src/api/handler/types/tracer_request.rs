//! [`TracerRequest`] — input for [`ObserverContext::tracer`](crate::api::handler::traits::ObserverContext::tracer).

/// Marker request; `tracer` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default)]
pub struct TracerRequest;
