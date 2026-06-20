//! `StdLifecycleFactory` — zero-sized marker for the standard lifecycle factory.

/// Zero-sized marker type that implements [`LifecycleBootstrap`](crate::api::lifecycle::traits::LifecycleBootstrap).
///
/// Obtain a concrete factory via `StdLifecycleFactory::std_factory()` or by
/// constructing the literal `StdLifecycleFactory`.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdLifecycleFactory;
