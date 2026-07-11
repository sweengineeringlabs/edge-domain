//! SAF — `DomainRuntime` service facade.
//!
//! `DomainRuntime` itself is re-exported publicly via `api::*` in lib.rs;
//! consumers import it from the crate root, not from this construction file.
/// SAF module anchor — satisfies arch-audit rule 221.
pub const DOMAIN_RUNTIME_SVC: () = ();
