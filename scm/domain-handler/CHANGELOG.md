# Changelog — edge-domain-handler

## [0.1.2] — 2026-07-16

### Changed
- **Breaking:** `Handler`/`HandlerRegistry`'s `Request`/`Response` associated types, and the
  local `Service`/`ServiceRegistry` mirror traits' `Request`/`Response`, are now bound against
  the shared `edge_application_base::{Request, Response}` contract instead of bare
  `Send + 'static` (issue #139). Any type used as a payload for these traits must now
  `impl edge_application_base::Request` / `impl edge_application_base::Response` for itself —
  `Send + 'static` alone is no longer sufficient.
- Added `edge-application-base` as a new required dependency.

### Fixed
- Backfilled this changelog for the `0.1.1` version bump, which shipped without an entry.

## [0.1.0] — 2026-06-12

### Added
- Initial extraction from `edge-domain` monolith (issue #21)
- `Handler` trait: request/response execution units with registry and context
