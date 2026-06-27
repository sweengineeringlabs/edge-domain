//! [`SecretString`] — a string whose value is never exposed via `Debug` or `Display`.

/// A heap-allocated string that redacts its contents in debug output.
///
/// Returned by [`CredentialResolver::resolve`](crate::CredentialResolver::resolve) so raw secrets never appear in
/// logs or error messages. Use [`SecretString::expose`] only at the point of
/// transmission and only within the transport layer.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretString(pub(crate) String);

