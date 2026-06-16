//! `StdLifecycleFactory` — zero-sized marker for the standard lifecycle factory.

/// Zero-sized marker type that implements [`LifecycleFactory`](crate::api::lifecycle::traits::LifecycleFactory).
///
/// Obtain a concrete factory via `StdLifecycleFactory::std_factory()` or by
/// constructing the literal `StdLifecycleFactory`.
#[derive(Debug, Clone, Copy, Default)]
pub struct StdLifecycleFactory;
