//! `NoopDomainExtension` — default no-op extension placeholder.

/// Default no-op extension. Satisfies [`crate::api::extension::traits::DomainExtension`]
/// without altering any behaviour. Use when no downstream extension is needed.
pub struct NoopDomainExtension;
