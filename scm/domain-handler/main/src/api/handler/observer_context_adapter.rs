//! [`ObserverContextAdapter`] — wraps an already-erased `&dyn Trait` reference
//! so it can bridge into the local [`ObserverContext`](crate::api::handler::traits::ObserverContext).
//!
//! Fully generic (no foreign type referenced here) so it satisfies SEA
//! `no_foreign_type`; the actual bridging `impl` lives in `core/` where
//! referencing `edge_application_observer::ObserverContext` is permitted.

/// Wraps `&'a T` so a caller already holding an erased `&dyn ForeignTrait`
/// (e.g. from `Box<dyn ForeignTrait>::as_ref()`) can bridge it into
/// [`ObserverContext`](crate::api::handler::traits::ObserverContext).
pub struct ObserverContextAdapter<'a, T: ?Sized>(pub &'a T);
