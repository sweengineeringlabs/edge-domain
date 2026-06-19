//! `EchoProviderCompleter` — echo-mode adapter implementing `edge-llm-complete`'s `Completer` port.

/// Echo adapter that implements [`edge_llm_complete::Completer`] via this provider's
/// `EchoExecutionModel`.
///
/// Returned by [`ProviderFactory::provider_completer`](crate::api::ProviderFactory).
#[derive(Debug, Clone, Copy, Default)]
pub struct EchoProviderCompleter;
