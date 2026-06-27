pub mod errors;
pub mod traits;
pub mod types;

// Trait contracts (from moved security-strategy)
pub use traits::AuthzPolicy;
pub use traits::CredentialResolver;
pub use traits::CredentialSourceResolver;
pub use traits::TokenVerifier;
pub use traits::Validator;

// Type contracts (from moved security-strategy)
pub use types::Claims;
pub use types::CredentialSource;
pub use types::CredentialSourceConfig;
pub use types::SecretString;
pub use types::Token;

// Original domain-security exports
pub use errors::SecurityError;
pub use errors::ValidationError;
pub use traits::Principal;
pub use traits::Security;
pub use traits::SecurityBootstrap;
pub use types::AnonymousPrincipal;
pub use types::NoopSecurity;
pub use types::SecurityContext;
pub use types::SecurityContextBuilder;
pub use types::SecurityServices;
