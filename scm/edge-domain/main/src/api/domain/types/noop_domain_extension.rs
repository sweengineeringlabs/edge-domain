//! `NoopDomainExtension` — default no-op extension placeholder.

use crate::api::domain::traits::DomainExtension;

/// Default no-op extension. Satisfies [`DomainExtension`]
/// without altering any behaviour. Use when no downstream extension is needed.
pub struct NoopDomainExtension;

impl DomainExtension for NoopDomainExtension {}
