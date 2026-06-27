//! [`AnonymousPrincipal`] — unauthenticated-caller sentinel.

/// Reference [`Principal`](crate::Principal) for unauthenticated requests.
///
/// The constants [`AnonymousPrincipal::ID`] and [`AnonymousPrincipal::KIND`] and all
/// methods are defined in `core::security::anonymous_principal`. This type is only a
/// declaration; all implementation is in the implementation layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnonymousPrincipal;
