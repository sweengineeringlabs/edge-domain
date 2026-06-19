//! `NoopCompleter` — reference [`Completer`](crate::api::complete::traits::Completer) that always errors.

/// Reference completer that always returns [`CompleteError::ProviderNotFound`](crate::api::complete::errors::CompleteError::ProviderNotFound).
///
/// Useful as a placeholder in wiring and as a negative test fixture.
#[derive(Clone, Debug, Default)]
pub struct NoopCompleter;
