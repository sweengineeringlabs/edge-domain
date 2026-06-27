//! # edge-domain-security
//!
//! The `Security` port contract — caller identity and context enforcement.
//!
//! Consolidates all security primitives and trait contracts:
//! - Principal identity modeling
//! - SecurityContext carrier
//! - Token verification contracts (TokenVerifier, Claims)
//! - Credential resolution (CredentialResolver, CredentialSourceResolver)
//! - Authorization enforcement (AuthzPolicy)

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

// Trait contracts (providers implement these)
pub use api::AuthzPolicy;
pub use api::CredentialResolver;
pub use api::CredentialSourceResolver;
pub use api::TokenVerifier;
pub use api::Validator;

// Type contracts (used in trait signatures)
pub use api::Claims;
pub use api::CredentialSource;
pub use api::CredentialSourceConfig;
pub use api::SecretString;
pub use api::Token;
pub use api::ValidationError;

// Original domain-security exports
pub use api::AnonymousPrincipal;
pub use api::NoopSecurity;
pub use api::SecurityContext;
pub use api::SecurityContextBuilder;
pub use api::SecurityError;
pub use api::SecurityServices;
pub use saf::Principal;
pub use saf::Security;
pub use saf::SecurityBootstrap;

// Service factory constants
pub use saf::PRINCIPAL_SVC_FACTORY;
pub use saf::SECURITY_BOOTSTRAP_SVC_FACTORY;
pub use saf::SECURITY_SVC_FACTORY;
pub use saf::TOKEN_VERIFIER_SVC_FACTORY;
pub use saf::VALIDATOR_SVC_FACTORY;
pub use saf::CREDENTIAL_RESOLVER_SVC_FACTORY;
pub use saf::CREDENTIAL_RESOLVER_SVC;
pub use saf::AUTHZ_POLICY_SVC_FACTORY;
pub use saf::CREDENTIAL_SOURCE_RESOLVER_SVC_FACTORY;
pub use saf::CREDENTIAL_SOURCE_RESOLVER_SVC;
