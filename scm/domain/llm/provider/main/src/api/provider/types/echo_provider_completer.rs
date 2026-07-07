//! `EchoProviderCompleter` — echo-mode adapter implementing `edge-llm-complete`'s `Completer` port.

/// Echo adapter that implements [`edge_llm_complete::Completer`] via this provider's
/// `EchoExecutionModel`.
///
/// Construct directly via [`EchoProviderCompleter`] unit-struct literal.
///
/// Orphan-type note: implements a trait from a *different* crate (`edge_llm_complete::Completer`),
/// which `no_orphan_types` doesn't see since it only scans this crate's own `api/traits/`. Not
/// fixable by adding a same-crate trait reference without misrepresenting what this type is for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EchoProviderCompleter;
