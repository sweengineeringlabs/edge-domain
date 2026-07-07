//! `StdPromptFactory` — constructs dispatchable prompt-rendering [`Handler`](edge_domain_handler::Handler)s.

/// Constructs dispatchable prompt-rendering handlers from a [`Prompt`](crate::api::Prompt)
/// (see `StdPromptFactory::prompt_handler`/`default_prompt_handler` in `core/`).
///
/// Orphan-type note: exposes its behavior via inherent methods that return `impl Handler`,
/// not by implementing a trait itself, so `no_orphan_types` flags it as unreferenced.
/// Implementing a marker trait purely to satisfy the check would be ceremony with no real
/// polymorphism behind it — the actual contract (`Handler`) is already satisfied by its
/// *return* type, just not by `StdPromptFactory` itself.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdPromptFactory;
