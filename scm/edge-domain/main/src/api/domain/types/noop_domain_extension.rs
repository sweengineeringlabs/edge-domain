//! `NoopDomainExtension` — default no-op extension placeholder.

/// Default no-op extension. Satisfies [`DomainExtension`](crate::api::DomainExtension)
/// without altering any behaviour. Use when no downstream extension is needed.
pub struct NoopDomainExtension;
