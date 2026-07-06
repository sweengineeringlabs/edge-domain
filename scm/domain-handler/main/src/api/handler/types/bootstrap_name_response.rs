//! [`BootstrapNameResponse`] — response for `bootstrap_name` on
//! [`HandlerBootstrap`](crate::api::handler::traits::HandlerBootstrap) and
//! [`HandlerProvider`](crate::api::handler::traits::HandlerProvider).

/// A bootstrap/provider implementation's stable identifier.
pub struct BootstrapNameResponse {
    /// The stable, non-empty identifier.
    pub name: &'static str,
}
