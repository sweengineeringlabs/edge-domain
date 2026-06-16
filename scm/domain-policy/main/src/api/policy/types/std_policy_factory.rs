//! `StdPolicyFactory` — the canonical [`PolicyFactory`](crate::PolicyFactory) marker.

/// Canonical marker that implements the standard [`PolicyFactory`](crate::PolicyFactory) contract.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdPolicyFactory;
