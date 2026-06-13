//! [`Principal`] — caller identity contract.

/// Caller identity — the authenticated (or anonymous) entity making a request.
///
/// Implement this for tenant identities, peer certificates, service accounts,
/// or anonymous sentinels.  Use [`AnonymousPrincipal`](crate::AnonymousPrincipal)
/// when no authentication is present.
pub trait Principal: Send + Sync {
    /// Return the unique identity string for this principal.
    fn id(&self) -> &str;

    /// Return the principal kind, e.g. `"tenant"`, `"service"`, `"anonymous"`.
    fn kind(&self) -> &str;
}
