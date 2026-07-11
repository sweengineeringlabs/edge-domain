//! `Domain` — factory type for domain building-block constructors.

/// Domain building-block factory.
///
/// A zero-size unit struct, so the type itself is a valid value: constructors
/// are called instance-based as `Domain.echo_handler(...)` rather than
/// `Domain::echo_handler(...)`. The non-generic constructors (those needing
/// no caller-supplied type parameters) are also exposed through the
/// [`DomainRuntime`](crate::api::DomainRuntime) trait, implemented on
/// `Domain` in `core/domain/domain_svc.rs`, giving callers a real
/// `dyn DomainRuntime` injection seam. Constructors that need generics
/// (`new_in_memory_repository::<T, Id>`, `echo_handler::<T>`, ...) stay
/// inherent methods — a generic method cannot be part of an object-safe
/// trait.
pub struct Domain;
