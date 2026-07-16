# Changelog — edge-domain-service

## [0.1.1] — 2026-07-16

### Added
- `NoopRequest`/`NoopResponse`: real, locally-owned zero-sized payload types for `NoopService`
  (replacing `()`, which can no longer satisfy `Request`/`Response` — see below).

### Changed
- **Breaking:** `Service`/`ServiceRegistry`'s `Request`/`Response` associated types are now
  bound against the shared `edge_application_base::{Request, Response}` contract instead of
  bare `Send + 'static` (issue #139). Any type used as a payload for these traits must now
  `impl edge_application_base::Request` / `impl edge_application_base::Response` for itself —
  `Send + 'static` alone, including primitive stand-ins like `()`/`String`, is no longer
  sufficient.
- Added `edge-application-base` as a new required dependency.

## [0.1.0] — 2026-06-12

### Added
- Initial extraction from `edge-domain` monolith (issue #21)
- `Service` trait: named domain operations with ServiceRegistry
