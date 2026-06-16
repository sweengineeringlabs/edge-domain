# Changelog

All notable changes to the `edge-domain-agent` crate are documented in this file.

## [0.1.0] - 2026-06-16

### Added
- Initial release: Agent domain primitives
- **Agent trait** — autonomous entity that executes skills
- **Skill trait** — named capability extending Handler; composable with ingress/egress
- **AgentManager trait** — loads and provides access to agents
- **AgentRegistry trait** — specializes Registry for agent discovery and metadata
- **AgentError enum** — errors for agent operations
- **AgentMetadata struct** — metadata for agent discovery
- **SkillMetadata struct** — metadata for skill discovery
- **Parameter struct** — documents skill input parameters
- Service constants for discovery: `AGENT_SVC`, `AGENT_MANAGER_SVC`, `AGENT_REGISTRY_SVC`

### Design

- Agents are first-class edge domain primitives (not plugins)
- Skills extend Handler for protocol-agnostic invocation
- Core layer is intentionally empty; implementations live in plugins
- Follows SEA (Structural Engineering Architecture) patterns

### References

- [ADR-030: Agent as Domain Primitive](../../docs/3-architecture/adr/ADR-030-agent-domain-primitive.md)
