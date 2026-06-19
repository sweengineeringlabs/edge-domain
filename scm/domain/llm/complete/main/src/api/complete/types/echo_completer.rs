//! `EchoCompleter` — reference [`Completer`](crate::api::complete::traits::Completer) that echoes input.

/// Reference completer that reflects the last user message back as the completion content.
///
/// Deterministic and dependency-free: suitable for unit tests and wiring smoke-tests.
#[derive(Clone, Debug, Default)]
pub struct EchoCompleter;
