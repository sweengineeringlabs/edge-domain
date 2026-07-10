//! Integration tests for the `edge_domain::Authenticator`/`Authorizer` re-exports.
#![cfg(feature = "security")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain::{
    AnonymousPrincipal, Authenticator, AuthnError, AuthnRequest, AuthnResponse, Authorizer,
    AuthzError, AuthzRequest, AuthzResponse, SecurityContext,
};
use edge_security_authn::Authenticator as RawAuthenticator;
use edge_security_authz::Authorizer as RawAuthorizer;

struct AlwaysAuthenticates;

#[async_trait::async_trait]
impl Authenticator for AlwaysAuthenticates {
    async fn authenticate(&self, req: AuthnRequest) -> Result<AuthnResponse, AuthnError> {
        let mut ctx = req.ctx;
        ctx.authenticated = true;
        ctx.principal = Some(Box::new(AnonymousPrincipal));
        Ok(AuthnResponse { ctx })
    }
}

struct RejectsUnlessBearerPresent;

#[async_trait::async_trait]
impl Authenticator for RejectsUnlessBearerPresent {
    async fn authenticate(&self, req: AuthnRequest) -> Result<AuthnResponse, AuthnError> {
        if req.ctx.token.is_none() {
            return Err(AuthnError::MissingToken);
        }
        let mut ctx = req.ctx;
        ctx.authenticated = true;
        Ok(AuthnResponse { ctx })
    }
}

/// @covers: Authenticator::authenticate
#[tokio::test]
async fn test_authenticate_always_authenticates_marks_context_happy() {
    let ctx = SecurityContext::unauthenticated();
    let response = AlwaysAuthenticates
        .authenticate(AuthnRequest { ctx })
        .await
        .unwrap();
    assert!(response.ctx.authenticated);
    assert!(response.ctx.principal.is_some());
}

/// @covers: Authenticator::authenticate
#[tokio::test]
async fn test_authenticate_missing_token_returns_error_error() {
    let ctx = SecurityContext::unauthenticated();
    let result = RejectsUnlessBearerPresent
        .authenticate(AuthnRequest { ctx })
        .await;
    assert!(matches!(result, Err(AuthnError::MissingToken)));
}

/// @covers: Authenticator::authenticate
#[tokio::test]
async fn test_authenticate_with_token_present_succeeds_edge() {
    let mut ctx = SecurityContext::unauthenticated();
    ctx.token = Some("valid-token".to_string());
    let response = RejectsUnlessBearerPresent
        .authenticate(AuthnRequest { ctx })
        .await
        .unwrap();
    assert!(response.ctx.authenticated);
}

struct AlwaysAuthorizes;

#[async_trait::async_trait]
impl Authorizer for AlwaysAuthorizes {
    async fn authorize(&self, req: AuthzRequest) -> Result<AuthzResponse, AuthzError> {
        let mut ctx = req.ctx;
        ctx.is_authorized = true;
        Ok(AuthzResponse { ctx })
    }
}

struct RequiresAuthenticatedPrincipal;

#[async_trait::async_trait]
impl Authorizer for RequiresAuthenticatedPrincipal {
    async fn authorize(&self, req: AuthzRequest) -> Result<AuthzResponse, AuthzError> {
        if req.ctx.principal.is_none() {
            return Err(AuthzError::MissingPrincipal);
        }
        let mut ctx = req.ctx;
        ctx.is_authorized = true;
        Ok(AuthzResponse { ctx })
    }
}

/// @covers: Authorizer::authorize
#[tokio::test]
async fn test_authorize_always_authorizes_marks_context_happy() {
    let ctx = SecurityContext::unauthenticated();
    let response = AlwaysAuthorizes
        .authorize(AuthzRequest { ctx })
        .await
        .unwrap();
    assert!(response.ctx.is_authorized);
}

/// @covers: Authorizer::authorize
#[tokio::test]
async fn test_authorize_missing_principal_returns_error_error() {
    let ctx = SecurityContext::unauthenticated();
    let result = RequiresAuthenticatedPrincipal
        .authorize(AuthzRequest { ctx })
        .await;
    assert!(matches!(result, Err(AuthzError::MissingPrincipal)));
}

/// @covers: Authorizer::authorize
#[tokio::test]
async fn test_authorize_with_principal_present_succeeds_edge() {
    let ctx = SecurityContext::authenticated_with(Box::new(AnonymousPrincipal));
    let response = RequiresAuthenticatedPrincipal
        .authorize(AuthzRequest { ctx })
        .await
        .unwrap();
    assert!(response.ctx.is_authorized);
}

/// @covers: edge_domain's re-exported Authenticator/Authorizer are the underlying
/// edge_security_authn/edge_security_authz traits, not look-alike wrappers.
#[test]
fn test_facade_authenticator_authorizer_are_the_raw_traits_edge() {
    fn assert_authenticator<T: Authenticator + RawAuthenticator>() -> bool {
        true
    }
    fn assert_authorizer<T: Authorizer + RawAuthorizer>() -> bool {
        true
    }
    assert!(assert_authenticator::<AlwaysAuthenticates>());
    assert!(assert_authorizer::<AlwaysAuthorizes>());
}
