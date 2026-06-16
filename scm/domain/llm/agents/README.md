# edge-domain-agent

Agent domain primitives: `Agent`, `Skill`, `AgentManager`, `AgentRegistry`.

Agents are first-class domain concepts in the edge framework. This crate defines the contracts; concrete implementations live in plugins (e.g., `edge-plugin-llmboot`).

## Traits

- **Agent** — Autonomous entity that pursues goals through skill execution
- **Skill** — Named capability that extends Handler; can be invoked over HTTP, gRPC, or async queues
- **AgentManager** — Service for loading and providing access to agents
- **AgentRegistry** — Specializes the generic Registry for agent discovery and metadata

## Architecture

```
api/
├── traits/        # Agent, Skill, AgentManager, AgentRegistry contracts
├── types/         # AgentMetadata, SkillMetadata
└── error/         # AgentError

core/
└── (empty — implementations live in plugins)

saf/
└── agent/         # Service re-exports via *_svc.rs files
```

## Building

```bash
cd domain-agent
cargo build
cargo test
cargo clippy -- -D warnings
```

## Documentation

- [ADR-030: Agent as Domain Primitive](../../docs/3-architecture/adr/ADR-030-agent-domain-primitive.md)
- [ADR-028: Plugin Tier](../../docs/3-architecture/adr/ADR-028-domain-plugin-tier.md)

## Implementation

Agent primitives are implemented by plugins:
- **edge-plugin-llmboot** — LLM-based agent execution with ReAct, CoT, and Plan-Execute patterns
