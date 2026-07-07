//! `StdReasoningFactory` — constructs dispatchable reasoning [`Handler`](edge_domain_handler::Handler)s.

/// Constructs dispatchable reasoning-execution handlers from a [`Reasoning`](crate::api::Reasoning)
/// (see `StdReasoningFactory::reasoning_handler`/`default_reasoning_handler` in `core/`).
///
/// Orphan-type note: exposes its behavior via inherent methods that return `impl Handler`,
/// not by implementing a trait itself, so `no_orphan_types` flags it as unreferenced — same
/// rationale as `edge-llm-prompt`'s `StdPromptFactory` and `edge-llm-provider`'s `StdProviderFactory`.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdReasoningFactory;
