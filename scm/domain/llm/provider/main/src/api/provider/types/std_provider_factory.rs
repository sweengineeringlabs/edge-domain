//! `StdProviderFactory` — reference [`ProviderFactory`](crate::api::provider::traits::ProviderFactory) implementation.

/// Reference implementation of [`ProviderFactory`](crate::api::provider::traits::ProviderFactory).
///
/// Implement the factory trait on this unit struct to gain the standard
/// constructors for the default provider primitives.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdProviderFactory;
