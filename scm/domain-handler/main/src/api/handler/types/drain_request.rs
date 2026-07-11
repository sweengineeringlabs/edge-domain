//! [`DrainRequest`] — input for [`ObserverContext::drain`](crate::api::handler::traits::ObserverContext::drain).

/// Marker request; `drain` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default)]
pub struct DrainRequest;
