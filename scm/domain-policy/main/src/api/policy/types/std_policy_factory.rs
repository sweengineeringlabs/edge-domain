//! `StdPolicyFactory` — the canonical [`PolicyBootstrap`](crate::PolicyBootstrap) marker.

/// Canonical marker that implements the standard [`PolicyBootstrap`](crate::PolicyBootstrap) contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdPolicyFactory;
