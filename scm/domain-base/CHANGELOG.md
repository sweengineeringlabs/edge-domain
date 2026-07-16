# Changelog — edge-domain-base

## [0.1.0] — 2026-07-16

### Added
- New crate (issue #139): `Request`/`Response` marker traits shared by `domain-handler` and
  `domain-service`, replacing each crate's independently-declared `Send + 'static` bound
