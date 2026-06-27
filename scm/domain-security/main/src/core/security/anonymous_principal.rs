//! `Principal` impl and inherent methods for `AnonymousPrincipal`.

use crate::{AnonymousPrincipal, Principal};

impl AnonymousPrincipal {
    /// Identity string returned by [`Principal::id`](crate::Principal::id).
    pub(crate) const ID: &'static str = "anonymous";
    /// Kind string returned by [`Principal::kind`](crate::Principal::kind).
    pub(crate) const KIND: &'static str = "anonymous";
}

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
