//! `Domain` — factory type for domain building-block constructors.

/// Domain building-block factory.
///
/// All constructors for domain infrastructure types live here as static
/// methods. Consumers call `Domain::echo_handler(...)` rather than
/// importing free functions.
///
/// Orphan-type note: exposes its behavior via inherent factory methods (see
/// `core/domain/domain_svc.rs`), not by implementing a trait, so
/// `no_orphan_types` flags it as unreferenced — accepted tradeoff, same
/// rationale as `edge-llm-reasoning`'s `StdReasoningFactory`.
pub struct Domain;
