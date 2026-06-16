pub use crate::api::security::AnonymousPrincipal;
pub use crate::api::security::Principal;

/// The anonymous principal singleton — a ready-made [`AnonymousPrincipal`] instance.
pub const ANONYMOUS: AnonymousPrincipal = AnonymousPrincipal;
