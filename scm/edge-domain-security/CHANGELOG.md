# Changelog — edge-domain-security

## [0.1.0] — 2026-06-12

### Added

- `Principal` trait — caller identity contract (`id`, `kind`)
- `SecurityContext` — request-scoped carrier for principal, tenant, claims, trace id
- `Security` primary trait — guard contract for authentication/authorisation enforcement
- `SecurityFactory` — default-method factory for `NoopSecurity`, `AnonymousPrincipal`, and context builders
- `NoopSecurity` — null-object guard (always `Ok`)
- `AnonymousPrincipal` — unauthenticated-caller sentinel
- `SecurityError` — `MissingClaims`, `EmptyPrincipalId`, `Unauthenticated` variants
