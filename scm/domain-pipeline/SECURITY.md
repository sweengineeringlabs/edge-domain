# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in edge-domain-pipeline, please report it to the SWE Labs security team via:

- **Email:** security@swelabs.io
- **GitHub Security Advisory:** [Report a vulnerability](https://github.com/anthropics/edge/security/advisories)

Please do not open public GitHub issues for security vulnerabilities.

## Security Practices

### Code Safety

- **No unsafe code** — All code is marked with `#![deny(unsafe_code)]`
- **Type safety** — Generic over context; no stringly-typed APIs
- **Error handling** — Fail-fast with explicit error types, no panics
- **Resource cleanup** — No resource leaks (Arc-managed lifetimes)

### Input Validation

- Steps validate their inputs at system boundaries
- PipelineError provides clear, non-leaking error messages
- Configuration validated at pipeline build time

### Dependencies

All dependencies are:
- Audited via `cargo audit`
- Pinned to specific versions in workspace
- Regularly updated for security patches

### Testing

- 109 comprehensive tests covering happy/error/edge paths
- No test mocks or stubs in production code
- Security-relevant behaviors validated end-to-end

## Known Limitations

- **Execution model**: Synchronous, sequential-only (no parallel execution)
- **Timeouts**: Timeout configuration supported; enforcement depends on step implementation
- **Cancellation**: No built-in cancellation mechanism (future enhancement)

## Compliance

- Follows SEA (Service Encapsulation Architecture) pattern
- Complies with production-grade engineering standards
- Regular security audits and dependency updates

## Contact

Security issues should be reported confidentially. See "Reporting Security Vulnerabilities" above.
