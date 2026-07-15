//! [`IntoHandlerResponse`] — response for [`IntoHandler::into_handler`](crate::api::handler::traits::IntoHandler::into_handler).

/// The handler produced by converting a domain service into a dispatch-pipeline handler.
///
/// Generic over the concrete handler type so conversion stays statically
/// dispatched — no trait-object boxing across the `Handler` + `ServiceHandler`
/// bound is required.
pub struct IntoHandlerResponse<H> {
    /// The converted handler.
    pub handler: H,
}
