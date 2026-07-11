//! [`CommandBusAdapter`] — wraps an already-erased `&dyn Trait` reference
//! so it can bridge into the local [`CommandBus`](crate::api::handler::traits::CommandBus).
//!
//! Fully generic (no foreign type referenced here) so it satisfies SEA
//! `no_foreign_type`; the actual bridging `impl` lives in `core/` where
//! referencing `edge_domain_command::CommandBus` is permitted.

/// Wraps `&'a T` so a caller already holding an erased `&dyn ForeignTrait`
/// can bridge it into [`CommandBus`](crate::api::handler::traits::CommandBus).
pub struct CommandBusAdapter<'a, T: ?Sized>(pub &'a T);
