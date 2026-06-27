//! [`CredentialSource`] — identifies an outbound credential by name.

/// Names the credential to fetch for an outbound call.
///
/// Infrastructure implementations look up the secret in their backing store
/// (Vault, Secrets Manager, env vars) using this identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CredentialSource {
    pub(crate) name: String,
}
