# Security Policy

## Reporting Security Vulnerabilities

If you discover a security vulnerability in `edge-domain-agent`, please email:

**security@sweengineeringlabs.io**

Do not create public GitHub issues for security vulnerabilities.

## Supported Versions

| Version | Status |
|---------|--------|
| 0.1.x   | Active |

## Security Considerations

### For Plugin Implementers

When implementing Agent, Skill, or AgentManager:

1. **Validate input** — All skill inputs must be validated before execution
2. **Error handling** — Avoid exposing stack traces or internal state in error messages
3. **Resource limits** — Implement timeouts and memory limits for long-running agents
4. **Audit logging** — Log agent invocations for security auditing
5. **Access control** — Enforce authentication/authorization at the ingress layer, not in agents

### For Edge Framework Users

- Register agents only through trusted plugin implementations
- Apply middleware (auth, rate-limiting) to all agent endpoints
- Monitor agent execution logs for anomalous behavior
- Keep agent specifications in secure, version-controlled storage
