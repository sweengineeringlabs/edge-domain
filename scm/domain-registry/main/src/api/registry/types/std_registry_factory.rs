//! `StdRegistryFactory` — reference implementation of [`RegistryFactory`].

/// Reference implementation of [`RegistryFactory`](crate::RegistryFactory).
/// Implement this trait on any unit struct to gain the standard registry
/// constructors.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdRegistryFactory;
