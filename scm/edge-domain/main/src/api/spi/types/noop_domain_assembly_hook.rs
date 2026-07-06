//! `NoopDomainAssemblyHook` — default no-op assembly hook placeholder.

/// Default no-op assembly hook. Satisfies
/// [`DomainAssemblyHook`](crate::api::DomainAssemblyHook) without registering
/// any custom domain assembly behaviour.
pub struct NoopDomainAssemblyHook;
