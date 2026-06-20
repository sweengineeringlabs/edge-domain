# edge-domain-security

The `Security` port contract — caller identity and context enforcement.

Part of the [swe-edge](https://github.com/sweengineeringlabs/edge-domain) domain layer.

## Overview

- `Principal` — caller identity trait (`id`, `kind`)
- `SecurityContext` — lean carrier for principal, tenant, claims, and trace id
- `Security` — primary guard trait; implement to enforce auth policies
- `SecurityBootstrap` — default-method bootstrap for standard implementations
- `NoopSecurity` — null-object guard for tests and open routes
- `AnonymousPrincipal` — sentinel for unauthenticated callers

## Usage

```rust
use edge_domain_security::{Security, SecurityBootstrap, SecurityContext};

struct AppSecurity;
impl SecurityBootstrap for AppSecurity {}

let guard = AppSecurity::noop_guard();
let ctx = AppSecurity::unauthenticated();
assert!(guard.enforce(&ctx).is_ok());
```
