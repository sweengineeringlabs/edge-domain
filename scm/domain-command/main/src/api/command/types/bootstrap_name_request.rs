//! [`BootstrapNameRequest`] — input for `CommandBootstrap::bootstrap_name` and
//! `CommandBusBootstrap::bootstrap_name`.

/// Request to resolve a bootstrap implementation's stable name.
///
/// Carries no data today; it exists so `bootstrap_name` conforms to the uniform
/// `*Request` -> `Result<T, E>` port-contract shape. Shared by
/// [`CommandBootstrap`](super::super::traits::CommandBootstrap) and
/// [`CommandBusBootstrap`](super::super::traits::CommandBusBootstrap).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameRequest;
