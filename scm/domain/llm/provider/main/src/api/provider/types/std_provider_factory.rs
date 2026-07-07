//! `StdProviderFactory` — constructs dispatchable provider-execution [`Handler`](edge_domain_handler::Handler)s.

/// Constructs dispatchable provider-execution handlers from an
/// [`ExecutionModel`](crate::api::ExecutionModel) (see `StdProviderFactory::provider_handler`/
/// `default_provider_handler` in `saf/provider/provider_handler_svc.rs`).
///
/// Orphan-type note: exposes its behavior via inherent methods that return `impl Handler`,
/// not by implementing a trait itself, so `no_orphan_types` flags it as unreferenced — same
/// rationale as `edge-llm-prompt`'s `StdPromptFactory`.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdProviderFactory;
