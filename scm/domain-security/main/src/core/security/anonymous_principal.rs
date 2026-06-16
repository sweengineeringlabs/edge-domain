//! `Principal` impl for `AnonymousPrincipal`.

use crate::api::security::traits::principal::Principal;
use crate::api::security::types::anonymous_principal::AnonymousPrincipal;

impl Principal for AnonymousPrincipal {
    fn id(&self) -> &str {
        AnonymousPrincipal::ID
    }

    fn kind(&self) -> &str {
        AnonymousPrincipal::KIND
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_returns_expected_constant() {
        assert_eq!(AnonymousPrincipal.id(), AnonymousPrincipal::ID);
    }

    #[test]
    fn test_kind_returns_expected_constant() {
        assert_eq!(AnonymousPrincipal.kind(), AnonymousPrincipal::KIND);
    }
}
