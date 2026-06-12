//! [`SecurityFactory`] — constructor contract for security objects.

use std::collections::HashMap;

use crate::api::security::errors::SecurityError;
use crate::api::security::traits::principal::Principal;
use crate::api::security::types::anonymous_principal::AnonymousPrincipal;
use crate::api::security::types::noop_security::NoopSecurity;
use crate::api::security::types::security_context::SecurityContext;
use crate::api::security::types::security_context_builder::SecurityContextBuilder;
use crate::api::security::types::security_services::SecurityServices;

/// Factory trait for the standard security implementations.
///
/// All methods have default implementations; call `impl SecurityFactory for
/// MyStruct {}` to get them for free.
pub trait SecurityFactory {
    /// Construct an unauthenticated [`SecurityContext`] with no principal.
    fn unauthenticated() -> SecurityContext {
        SecurityContext::unauthenticated()
    }

    /// Construct an authenticated [`SecurityContext`] for the given principal.
    fn authenticated(principal: Box<dyn Principal>) -> SecurityContext {
        SecurityContext::authenticated_with(principal)
    }

    /// Construct a [`SecurityContext`] from a claims map.
    ///
    /// Returns [`SecurityError::MissingClaims`] when `claims` is empty.
    fn from_claims(claims: HashMap<String, String>) -> Result<SecurityContext, SecurityError> {
        if claims.is_empty() {
            return Err(SecurityError::MissingClaims);
        }
        let mut ctx = SecurityContext::unauthenticated();
        ctx.claims = claims;
        Ok(ctx)
    }

    /// Construct a [`NoopSecurity`] guard that allows every context.
    fn noop_guard() -> NoopSecurity {
        NoopSecurity
    }

    /// Construct an [`AnonymousPrincipal`] reference implementation.
    fn anonymous_principal() -> AnonymousPrincipal {
        AnonymousPrincipal
    }

    /// Return the default [`SecurityServices`] factory implementation.
    fn default_services() -> SecurityServices {
        SecurityServices
    }

    /// Return a fresh [`SecurityContextBuilder`] with no fields set.
    fn context_builder() -> SecurityContextBuilder {
        SecurityContextBuilder::new()
    }
}
