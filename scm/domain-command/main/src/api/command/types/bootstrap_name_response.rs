//! [`BootstrapNameResponse`] — output of `CommandBootstrap::bootstrap_name` and
//! `CommandBusBootstrap::bootstrap_name`.

/// Resolved stable name of a bootstrap implementation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BootstrapNameResponse {
    /// The bootstrap implementation's stable name.
    pub name: &'static str,
}
