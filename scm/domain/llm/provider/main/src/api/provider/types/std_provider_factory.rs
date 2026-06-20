//! `StdProviderFactory` — reference [`ProviderBootstrap`](crate::api::provider::traits::ProviderBootstrap) implementation.

/// Reference implementation of [`ProviderBootstrap`](crate::api::provider::traits::ProviderBootstrap).
///
/// Implement the factory trait on this unit struct to gain the standard
/// constructors for the default provider primitives.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdProviderFactory;
