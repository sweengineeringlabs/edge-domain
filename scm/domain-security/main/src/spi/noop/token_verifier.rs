//! Noop [`TokenVerifier`] implementation.

use crate::api::Claims;
use crate::api::SecurityError;
use crate::api::TokenVerifier;

/// Noop token verifier that rejects all tokens.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NoopTokenVerifier;

impl TokenVerifier for NoopTokenVerifier {
    fn verify(&self, _token: &str) -> Result<Claims, SecurityError> {
        Err(SecurityError::Auth(
            "noop verifier rejects all tokens".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_token_verifier_rejects_all() {
        let verifier = NoopTokenVerifier;
        assert!(verifier.verify("test-token").is_err());
    }
}
