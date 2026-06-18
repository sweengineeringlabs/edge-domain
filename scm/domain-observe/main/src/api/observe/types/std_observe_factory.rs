//! `StdObserveFactory` — the standard (noop) observe factory.

/// The concrete [`ObserveFactory`] backed by noop primitives.
///
/// Wire SDK-backed implementations at the assembler layer for production.
///
/// [`ObserveFactory`]: crate::ObserveFactory
pub struct StdObserveFactory;
