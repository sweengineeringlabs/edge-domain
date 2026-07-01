//! [`HandlerBuildResponse`] — response for [`HandlerBootstrap::build`](crate::api::handler::traits::HandlerBootstrap::build).

/// The handler constructed by [`HandlerBootstrap::build`](crate::api::handler::traits::HandlerBootstrap::build).
///
/// Generic over the concrete handler type so construction stays statically
/// dispatched.
#[derive(Debug)]
pub struct HandlerBuildResponse<H> {
    /// The constructed handler.
    pub handler: H,
}
