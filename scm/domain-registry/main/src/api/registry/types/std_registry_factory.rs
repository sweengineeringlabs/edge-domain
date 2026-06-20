//! `StdRegistryFactory` — reference implementation of [`RegistryBootstrap`].

/// Reference implementation of [`RegistryBootstrap`](crate::RegistryBootstrap).
/// Implement this trait on any unit struct to gain the standard registry
/// constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdRegistryFactory;
