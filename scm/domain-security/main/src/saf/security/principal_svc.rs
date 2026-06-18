pub use crate::api::AnonymousPrincipal;
pub use crate::api::Principal;

/// The anonymous principal singleton — a ready-made [`AnonymousPrincipal`] instance.
pub const ANONYMOUS: AnonymousPrincipal = AnonymousPrincipal;
