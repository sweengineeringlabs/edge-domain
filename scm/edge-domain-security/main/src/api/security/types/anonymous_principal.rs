//! [`AnonymousPrincipal`] — unauthenticated-caller sentinel.

/// Reference [`Principal`](crate::Principal) for unauthenticated requests.
///
/// Returns [`AnonymousPrincipal::ID`] for `id` and [`AnonymousPrincipal::KIND`] for `kind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnonymousPrincipal;

impl AnonymousPrincipal {
    /// Identity string returned by [`Principal::id`](crate::Principal::id).
    pub const ID: &'static str = "anonymous";
    /// Kind string returned by [`Principal::kind`](crate::Principal::kind).
    pub const KIND: &'static str = "anonymous";
}
