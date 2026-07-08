# ADR-046: `edge-llm-tools` — Tool Governance Domain Primitive

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Supersedes (design, not intent):** [edge ADR-036](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-036-llm-tools.md) — LLM Tool Governance Domain. ADR-036's problem statement and capability/risk vocabulary still stand; its `GovernancePolicy`/`ExecutionContext`/`AuditEntry`/`ToolGovernanceError` design predates two things that now make it directly simplifiable: the Request/Response port mandate (arch 0.2.46) and the existing `edge-domain-policy`/`edge-security-runtime`/`edge-domain-observer` crates, none of which existed or were wired into `HandlerContext` when ADR-036 was drafted (2026-06-16).
**Relates to:** ADR-030 (Agent), ADR-032 (LLM Agent), ADR-033 (LLM Provider), ADR-035 (LLM Reasoning), ADR-044 (Observability↔LLM Integration), ADR-045 (`edge-llm-runtime` Composition Root)
**GitHub Issues:** TBD

---

## Context

ADR-036 (2026-06-16) proposed `edge_llm_tools`: capability declarations (`CapabilityFlags`, 12 bitmask flags), risk levels, and a `GovernancePolicy` trait so that "any skill an agent has runs unrestricted" (confirmed still true by the 2026-07-08 landscape audit — zero capability/risk/governance code exists anywhere in `edge-llm-agent` today) would no longer hold. It was never built: 0% implemented, zero references anywhere in the repo.

Since ADR-036 was written, three things changed underneath it:

1. **`edge-domain-policy`** now exists, with exactly the shape ADR-036's `GovernancePolicy` reinvented: `Policy<Input = I>::evaluate(req) -> Result<(), PolicyError>`, `CompositePolicy<I>` for AND-composing multiple rules, and `PolicyError { policy: &'static str, reason: String }` — already satisfying the `api_error_type_named` rule ADR-036's bespoke `ToolGovernanceError` would also have needed.
2. **`SecurityContext`** (`edge-security-runtime`) already carries `principal`, `tenant_id`, `claim(key)`, `metadata`, `trace_id` — already threaded into every `Handler::execute`/`Skill::execute` call via `HandlerContext.security`. This overlaps almost entirely with ADR-036's bespoke `ExecutionContext { agent_id, user_id, tenant_id, timestamp }`.
3. **`ObserverContext`** (`edge-domain-observer`) is already threaded into `HandlerContext.observer` and already proven wired into `DefaultAgentHandler`/`Provider` (ADR-044). ADR-036's bespoke `AuditEntry` (a logging-only struct with no consumer) duplicates what a tracer span + counter already give for free.
4. `edge-llm-agent`'s `Skill: Handler` trait already carries `ParameterDocumentation`/`ToolMetadata`-shaped types (`ParameterDocumentationBuilder`, `SkillMetadata`) — ADR-036's `ToolMetadata`/`ParameterDocumentation` types would duplicate these rather than reuse them.

So the governance gap is real and unchanged; the *design* to close it should reuse what now exists instead of building three parallel ports (`Policy`-shaped, `SecurityContext`-shaped, `ObserverContext`-shaped) that already have a home.

## Decision

Build `edge-llm-tools` as a small crate, kept for the parts that are genuinely novel LLM/tool vocabulary, everything else composed from existing ports:

### Kept from ADR-036 (novel, no existing analog)

- **`CapabilityFlags`** — the 12-flag bitmask (`FILE_READ`, `FILE_WRITE`, `NETWORK_LOCAL`, `NETWORK_EXTERNAL`, `PROCESS_SPAWN`, `PROCESS_TERMINATE`, `DATABASE_READ`, `DATABASE_WRITE`, `WEBHOOK_SEND`, `API_CALL`, `MEMORY_ACCESS`, `SYSTEM_COMMAND`). Unchanged from ADR-036.
- **`RiskLevel`** (`None < Low < Medium < High < Critical`, `meets_requirement`). Unchanged.
- **`ToolCapabilityModel`** trait — what a tool declares about itself: id, capabilities, risk level. Converted to the mandatory Request/Response shape:
  ```rust
  pub trait ToolCapabilityModel: Send + Sync {
      fn tool_id(&self, req: ToolIdRequest) -> Result<ToolIdResponse, ToolCapabilityError>;
      fn capabilities(&self, req: CapabilityLookupRequest) -> Result<CapabilityLookupResponse, ToolCapabilityError>;
      fn risk_level(&self, req: RiskLevelRequest) -> Result<RiskLevelResponse, ToolCapabilityError>;
  }
  ```
  `ToolCapabilityError` is the one new error type this crate needs — for malformed capability declarations, not for policy decisions (see below). Reuses `edge_llm_agent::ParameterDocumentation`/`SkillMetadata` rather than redefining metadata types.

### Replaced with existing ports (the actual simplification vs. ADR-036)

- **No `GovernancePolicy` trait.** A tool-invocation gate is just `edge_domain_policy::Policy<Input = ToolInvocationRequest>`. `ToolInvocationRequest` bundles the `ToolCapabilityModel`'s declared capabilities/risk with the caller's `&SecurityContext` (already in scope at every call site via `HandlerContext.security` — no new context type). Capability checks (`InsufficientCapabilities`) and risk-ceiling checks (`RiskLevelExceeded`) are each just another `Policy` impl (e.g. `CapabilityGatePolicy`, `RiskCeilingPolicy`), composed with `CompositePolicy::new().with(...).with(...)` — AND semantics, first violation short-circuits, exactly ADR-036's intent, using the port that already exists. All governance failures surface as `PolicyError`, not a bespoke `ToolGovernanceError` enum.
- **No `ExecutionContext` struct.** Use `&SecurityContext` directly — `tenant_id`, `principal` (as agent/caller identity), `claim(...)`, `metadata`, `trace_id` already cover ADR-036's `agent_id`/`user_id`/`tenant_id`/`timestamp` fields. If a policy needs the invoking agent's id specifically, it reads it from a `SecurityContext` claim or metadata entry set at agent-dispatch time — no parallel context to keep in sync.
- **No `AuditEntry` struct.** Allow/deny decisions are recorded via the existing `ObserverContext` seam: a span attribute (`tool.governance.decision`) plus a counter increment (`tool_governance_checked_total{allowed=...}`), the same mechanism already proven for `DefaultAgentHandler`/`Provider` under ADR-044. This directly fixes ADR-036's own admitted limitation ("Audit is logging-only") by reusing infrastructure that's already wired end-to-end, instead of adding a fourth unconsumed struct.

### Enforcement point

A tool-invocation gate is enforced as a `Handler`-wrapping decorator — the same shape as `edge-dispatcher`'s `TimeoutHandler` (`impl<H: Handler> Handler for GovernedHandler<H>`), surfaced by the ADR-045 audit. `GovernedHandler<H>` holds the inner `Skill`/`Handler`, its `ToolCapabilityModel`, and a `CompositePolicy<ToolInvocationRequest>`; on `execute`, it evaluates the policy against `(capabilities, ctx.security)` before delegating to `self.inner.execute(...)`, returning a `HandlerError` wrapping `PolicyError` on denial.

This means **no changes to `edge-llm-agent`'s `Skill`/`Agent` traits at all** — governance is applied at registration time in the composition root (wrap the skill's handler in `GovernedHandler` before registering it with `RuntimeBuilder`/`HandlerRegistryImpl`, per ADR-045), not baked into the domain trait. Materially less invasive than ADR-036's original framing ("Agent has a capability set... cannot invoke tools outside it"), which implied changes inside `Agent`/`Skill` itself.

```
Skill::execute(req, ctx)              (unrestricted, as today)
        ▲
GovernedHandler<Skill>::execute(req, ctx)
    ├─► CompositePolicy.evaluate(ToolInvocationRequest{ capabilities, risk, security: ctx.security })
    │     ├─ CapabilityGatePolicy   → PolicyError on missing capability
    │     └─ RiskCeilingPolicy      → PolicyError on risk above tenant ceiling
    ├─► ctx.observer: span attr + counter (allow/deny)
    └─► on Ok: self.inner.execute(req, ctx)   on Err: HandlerError (deny, no execution)
```

### Workspace layout

```
domain/scm/domain/llm/tools/          (edge-llm-tools)
├── api/
│   ├── traits/tool_capability_model.rs
│   ├── types/{capability_flag,risk_level,tool_invocation_request}.rs
│   └── errors/tool_capability_error.rs
├── core/
│   ├── governed_handler.rs           (the decorator)
│   ├── capability_gate_policy.rs     (impl Policy<Input=ToolInvocationRequest>)
│   └── risk_ceiling_policy.rs        (impl Policy<Input=ToolInvocationRequest>)
└── saf/
```

Depends on: `edge-domain-handler` (`Handler`, `HandlerContext`, `HandlerError`), `edge-domain-policy` (`Policy`, `CompositePolicy`, `PolicyError`), `edge-security-runtime` (`SecurityContext`), `edge-domain-observer` (`ObserverContext`), `edge-llm-agent` (`ParameterDocumentation`/`SkillMetadata`, for `ToolCapabilityModel`'s optional metadata surface).

## Consequences

**What this enables**
- Any `Skill`/`Handler` registered through `edge-llm-runtime` (ADR-045) can be wrapped in `GovernedHandler` for a real capability/risk gate — closing the "any skill runs unrestricted" gap the 2026-07-08 audit found.
- Governance decisions are visible in the same observability plane as everything else (ADR-044), not a separate unconsumed audit log.
- Composable, pluggable policy per ADR-036's original intent (`CompositePolicy` — startup: allow most tools; enterprise: strict whitelist), with zero new composition machinery, since `CompositePolicy` already exists and is tested.
- Smaller crate than ADR-036 proposed: one new error type (`ToolCapabilityError`) instead of four new concepts (`GovernancePolicy`, `ExecutionContext`, `AuditEntry`, `ToolGovernanceError`).

**What this requires**
- New crate `edge-llm-tools` under `domain/scm/domain/llm/tools/`.
- `edge-llm-agent`'s skills need a `ToolCapabilityModel` declared per skill (new work — today no skill declares capabilities at all, so this is not just wiring, it's a real per-skill authoring task once the crate exists).
- No changes to `edge-domain-handler`, `edge-domain-policy`, `edge-security-runtime`, `edge-domain-observer`, or `edge-llm-agent`'s existing traits.

**What this does NOT solve**
- Fine-grained permissions (row-level DB access, path-scoped file access) — `CapabilityFlags` stays all-or-nothing per capability, same limitation ADR-036 named and left as future work.
- Capability delegation (agent granting a subset to a sub-agent) — not addressed here either.
- This ADR does not retrofit governance onto any *already-registered* handler automatically — each composition root (e.g. `edge-llm-runtime`) must explicitly choose to wrap a skill in `GovernedHandler`; an ungoverned registration is still possible and won't be flagged by tooling.

## Alternatives Considered

**Build ADR-036 exactly as originally spec'd (`GovernancePolicy`/`ExecutionContext`/`AuditEntry`/`ToolGovernanceError`)**
Rejected. Would duplicate `edge-domain-policy::Policy`, `SecurityContext`, and `ObserverContext` field-for-field with parallel types that don't compose with the rest of the domain (e.g. a bespoke `AuditEntry` nobody else's tooling reads). Every duplicated concept is also a duplicated set of arch-audit obligations (own error-naming rule, own test scenarios) for no behavioral gain.

**Bake capability checks directly into `Skill`/`Agent` (ADR-036's original framing)**
Rejected. Requires changing `edge-llm-agent`'s trait surface and every existing `Skill` impl. The decorator approach (`GovernedHandler<H>`) achieves the same gating by wrapping at registration time — zero changes to the domain crate, consistent with how `edge-dispatcher`'s `TimeoutHandler` already adds a cross-cutting concern the same way.

## Tracking

- New crate: `edge-llm-tools` (`domain/scm/domain/llm/tools/`)
- Depends on this ADR being accepted before any `Skill` declares a `ToolCapabilityModel` (no per-skill work should start against the old ADR-036 shape)
- Follow-up: audit whether `edge-llm-runtime` (ADR-045) should wrap *all* registered handlers in `GovernedHandler` by default, or require opt-in per handler
