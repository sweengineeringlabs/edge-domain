# ADR-073: Fine-Grained Capability Scoping + Sub-Agent Delegation — `edge-llm-tools` extension

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-030 (Agent), ADR-032 (LLM Agent), ADR-046 (`edge-llm-tools` Tool Governance, sibling — the exact contract this ADR extends, not redesigns)
**GitHub Issues:** TBD

---

## Context

ADR-046 built `CapabilityFlags` (a 12-flag bitmask: `FILE_READ`, `FILE_WRITE`, `NETWORK_LOCAL`, `NETWORK_EXTERNAL`, `PROCESS_SPAWN`, `PROCESS_TERMINATE`, `DATABASE_READ`, `DATABASE_WRITE`, `WEBHOOK_SEND`, `API_CALL`, `MEMORY_ACCESS`, `SYSTEM_COMMAND`), `RiskLevel`, `ToolCapabilityModel`, and enforced both via `GovernedHandler<H>` evaluating a `CompositePolicy<ToolInvocationRequest>` (`CapabilityGatePolicy` + `RiskCeilingPolicy`) at invocation time. ADR-046 named two gaps explicitly and left both as future work:

> "Fine-grained permissions (row-level DB access, path-scoped file access) — `CapabilityFlags` stays all-or-nothing per capability, same limitation ADR-036 named and left as future work."
> "Capability delegation (agent granting a subset to a sub-agent) — not addressed here either."

Per explicit user direction, nothing in this audit series stays merely "deferred" — every gap a prior ADR named as a follow-up gets its own full ADR. This is that ADR for both gaps named above. As of this writing `edge-llm-tools` itself has not been implemented (ADR-046 remains Proposed, and `domain/scm/domain/llm/tools/` does not exist on disk) — so everything below amends an unbuilt design, not shipped code. That matters for scope: there is no live `CapabilityLookupResponse`/`ToolInvocationRequest` to keep binary-compatible, only a design to get right before the first line of `edge-llm-tools` is written.

### The exact shapes this ADR extends — verified against ADR-046, not re-derived

```rust
// CapabilityFlags — ADR-046, unchanged bit layout, 12 flags, still all-or-nothing per flag
pub struct CapabilityFlags { /* bitmask: FILE_READ, FILE_WRITE, NETWORK_LOCAL, NETWORK_EXTERNAL,
                                 PROCESS_SPAWN, PROCESS_TERMINATE, DATABASE_READ, DATABASE_WRITE,
                                 WEBHOOK_SEND, API_CALL, MEMORY_ACCESS, SYSTEM_COMMAND */ }

// ToolCapabilityModel — ADR-046
pub trait ToolCapabilityModel: Send + Sync {
    fn tool_id(&self, req: ToolIdRequest) -> Result<ToolIdResponse, ToolCapabilityError>;
    fn capabilities(&self, req: CapabilityLookupRequest) -> Result<CapabilityLookupResponse, ToolCapabilityError>;
    fn risk_level(&self, req: RiskLevelRequest) -> Result<RiskLevelResponse, ToolCapabilityError>;
}

// ToolInvocationRequest bundles declared capabilities/risk + &SecurityContext — ADR-046
// (exact field list left implicit in ADR-046's prose; this ADR makes it concrete, see below)

// GovernedHandler<H> — ADR-046: evaluates CompositePolicy<ToolInvocationRequest> before
// delegating to self.inner.execute(req, ctx); denial => HandlerError wrapping PolicyError.
```

And the sub-agent construction path actually in the code today (`domain/scm/domain/llm/agents/main/src/api/traits/agent_manager.rs`, `agent.rs`):

```rust
// agent_manager.rs:32-36 — this is the ONLY place an Agent is constructed
fn default_agent(&self, req: AgentCreationRequest<'_>) -> Result<AgentCreationResponse, AgentError>;

// agent_creation_request.rs — no notion of "parent agent" or "delegated capability" exists
pub struct AgentCreationRequest<'a> {
    pub id: &'a str, pub name: &'a str, pub description: &'a str,
    pub provider: Arc<dyn Provider>,
    pub skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
}
```

Confirmed by grep across `domain/scm/domain/llm/`: there is no `spawn`, `sub_agent`, or `delegate` concept anywhere in the codebase today. `Agent::execute_skill` (`agent.rs:33-36`) runs a skill the agent already owns; it does not create other agents. A "sub-agent" is, structurally, nothing more than a second `AgentCreationRequest` fed a curated `skills: Vec<...>` — there is no separate spawn primitive to extend, only `default_agent` itself. Any delegation design must be honest about that: this ADR is not hardening an existing spawn path, it is introducing the first one.

## Decision

Extend `edge-llm-tools` (still unbuilt, per ADR-046) with two additive pieces. Both stay inside `edge-llm-tools` — no changes to `edge-llm-agent`'s `Agent`/`AgentManager`/`Skill` traits or to `AgentCreationRequest`, for the same reason ADR-046 kept `GovernedHandler` out of `Skill`/`Agent`: `edge-llm-tools` already depends on `edge-llm-agent` (one direction only, established in ADR-046 for `ParameterDocumentation`/`SkillMetadata` reuse); pushing `CapabilityFlags`/`Policy`-shaped types *into* `edge-llm-agent` would invert that dependency for no reason — every new type below has a home in the crate that already knows about capabilities and policies.

### Part 1 — Fine-grained (path-scoped) capability checking

**Scope for v1: `FILE_READ` and `FILE_WRITE` only.** These are the two capabilities where "the resource being accessed" is unambiguously a single string (a filesystem path) checkable against a glob-style pattern with no additional infrastructure. The other 10 flags either have no natural single-resource identifier at this layer (`PROCESS_SPAWN`, `MEMORY_ACCESS`, `SYSTEM_COMMAND`), or their scoping shape is a materially different, larger design (`DATABASE_READ`/`DATABASE_WRITE` row/table scoping needs a query-shape-aware matcher, not a glob; `NETWORK_EXTERNAL`/`WEBHOOK_SEND`/`API_CALL` URL scoping needs host/path/method matching against an actual outbound request). Named explicitly below as follow-ups, not designed here.

```rust
// api/types/capability_scope.rs — NEW
/// A refinement narrowing a coarse CapabilityFlags bit to a specific resource shape.
/// Absence of a scope for a given flag (an empty Vec below) means that flag stays
/// all-or-nothing, exactly ADR-046's original semantics — this is the backward-compatible
/// default for the 10 capabilities this ADR does not scope.
pub enum CapabilityScope {
    FileReadPaths(Vec<PathPattern>),
    FileWritePaths(Vec<PathPattern>),
}

// api/types/path_pattern.rs — NEW
/// A single glob-style path pattern (e.g. "/workspace/**", "/tmp/agent-7f3/**").
/// Matching semantics: a target path is allowed iff it matches at least one pattern.
pub struct PathPattern(pub String);

// api/types/target_resource.rs — NEW
/// The concrete resource a tool invocation is actually trying to touch, if the
/// invoked capability is scoped. `None` for every unscoped capability (10 of 12
/// in v1) and for any scoped capability a caller declares but doesn't populate —
/// see "fail-closed on missing resource" below.
pub enum TargetResource<'a> {
    Path(&'a std::path::Path),
    // Row(..), Url(..): named follow-ups, not built in v1 (see "does NOT solve").
}
```

`CapabilityLookupResponse` (ADR-046's `ToolCapabilityModel::capabilities` return type) gains one additive field:

```rust
pub struct CapabilityLookupResponse {
    pub flags: CapabilityFlags,           // unchanged from ADR-046
    pub scopes: Vec<CapabilityScope>,     // NEW — empty Vec = fully coarse-grained, ADR-046 behavior
}
```

`ToolInvocationRequest` (the `Policy<Input = ToolInvocationRequest>` input ADR-046 left implicit) is made concrete here, with one new optional field:

```rust
// api/types/tool_invocation_request.rs
pub struct ToolInvocationRequest<'a> {
    pub capabilities: CapabilityFlags,          // ADR-046, unchanged
    pub scopes: &'a [CapabilityScope],          // NEW — from the invoked tool's CapabilityLookupResponse
    pub risk: RiskLevel,                        // ADR-046, unchanged
    pub security: &'a SecurityContext,          // ADR-046, unchanged
    pub target_resource: Option<TargetResource<'a>>, // NEW — the actual path/row/URL this call touches
}
```

`target_resource` is `Option` deliberately: every tool that declares only unscoped flags (still 10 of 12 capability kinds, and 100% of tools until someone opts a `FILE_READ`/`FILE_WRITE` skill into scoping) passes `None`, and `CapabilityGatePolicy`'s existing bit-check runs exactly as ADR-046 specified — this field is invisible to them. Tools have to know their own input schema to extract a path from `req.input: String` (a serialized JSON blob per `SkillExecutionRequest`), so extraction cannot happen generically inside `GovernedHandler`. This is pushed to `ToolCapabilityModel` itself via one new, default-provided method (additive, does not break the trait for existing implementors — the same growable-trait shape `AgentManager` already uses for `conversation_loop`/`agent_metadata_builder`, `agent_manager.rs:38-69`):

```rust
pub trait ToolCapabilityModel: Send + Sync {
    // ...tool_id / capabilities / risk_level unchanged from ADR-046...

    /// Extract the concrete resource this invocation targets, if this tool declares
    /// a scoped capability. Default: no scoping declared, always None — unscoped
    /// tools never need to override this.
    fn resource_from_input(
        &self,
        _req: ResourceExtractionRequest<'_>,
    ) -> Result<ResourceExtractionResponse<'_>, ToolCapabilityError> {
        Ok(ResourceExtractionResponse { resource: None })
    }
}
```

`CapabilityGatePolicy::evaluate` (ADR-046) gains scoped-check logic, additive to its existing bit check:

```
for each scope in req.scopes:
    match scope:
      FileReadPaths(patterns)  if req.capabilities.contains(FILE_READ) =>
          require req.target_resource == Some(Path(p)) matching >=1 pattern
          else PolicyError("capability-gate", "FILE_READ scoped to {patterns}, target {p_or_missing} not allowed")
      FileWritePaths(patterns) if req.capabilities.contains(FILE_WRITE) => (symmetric)
if req.scopes is empty: behavior identical to ADR-046 (bit-presence check only)
```

**Fail-closed on missing resource, not fail-open**: if a tool declares a `FileReadPaths`/`FileWritePaths` scope but `resource_from_input` returns `None` (extraction failed, or the tool author didn't wire it), the gate denies rather than silently falling back to the coarse-grained "flag present = allowed" check — a scoped declaration is a stronger claim ("I only ever touch these paths") and an invocation that can't prove which path it's touching must not get the benefit of the doubt. This is the same fail-closed-on-ambiguity posture `RiskCeilingPolicy` already takes for missing risk data in ADR-046.

### Part 2 — Sub-agent capability delegation

**Core safety invariant, stated explicitly:** a sub-agent's granted `CapabilityFlags` must always be a subset of the delegating agent's own capabilities, and its risk ceiling must never exceed the delegating agent's — `requested ⊆ parent` on both axes, always, with no override. This is enforced, not merely documented, at the one and only place a new `Agent` is actually constructed: `AgentManager::default_agent`.

Because `default_agent`'s existing request (`AgentCreationRequest`) carries no notion of "parent" or "delegated subset," and because `CapabilityFlags`/`Policy` cannot be pulled into `edge-llm-agent` (dependency direction, see above), delegation is enforced by a **decorator around `AgentManager`** — the direct analog of `GovernedHandler` wrapping `Handler`, applied to the one construction seam that exists instead of a per-invocation one:

```rust
// api/types/delegation_request.rs — NEW
pub struct DelegationRequest {
    pub parent_capabilities: CapabilityFlags,
    pub requested_capabilities: CapabilityFlags,
    pub parent_risk_ceiling: RiskLevel,
    pub requested_risk_ceiling: RiskLevel,
}

// core/delegation_subset_policy.rs — NEW
// impl Policy<Input = DelegationRequest>, same shape as ADR-046's CapabilityGatePolicy/RiskCeilingPolicy
impl Policy for DelegationSubsetPolicy {
    type Input = DelegationRequest;
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, DelegationRequest>) -> Result<(), PolicyError> {
        let d = req.input;
        if !d.parent_capabilities.contains(d.requested_capabilities) {
            return Err(PolicyError { policy: "delegation-subset",
                reason: "sub-agent requested capabilities not held by parent".into() });
        }
        if d.requested_risk_ceiling > d.parent_risk_ceiling {
            return Err(PolicyError { policy: "delegation-subset",
                reason: "sub-agent risk ceiling exceeds parent's".into() });
        }
        Ok(())
    }
}

// api/errors/delegation_error.rs — NEW, the one new error type this half of the ADR needs
pub enum DelegationError {
    CapabilityDenied(PolicyError),
    AgentCreation(AgentError),  // propagated verbatim from the wrapped default_agent call
}

// core/delegating_agent_manager.rs — NEW
// Decorator: implements AgentManager (forwarding load_agent/agent/list_agent_ids/
// agent_handler/agent_metadata_builder/skill_metadata_builder/conversation_loop
// verbatim to `inner`, unchanged), and additionally exposes one new inherent method
// that is NOT part of the AgentManager trait (the trait itself never learns about
// CapabilityFlags — only this crate's decorator does):
impl<M: AgentManager> DelegatingAgentManager<M> {
    pub fn spawn_sub_agent(
        &self,
        req: SubAgentSpawnRequest<'_>,
    ) -> Result<AgentCreationResponse, DelegationError> {
        self.delegation_policy
            .evaluate(PolicyEvaluateRequest { input: &DelegationRequest {
                parent_capabilities: req.parent_capabilities,
                requested_capabilities: req.requested_capabilities,
                parent_risk_ceiling: req.parent_risk_ceiling,
                requested_risk_ceiling: req.requested_risk_ceiling,
            }})
            .map_err(DelegationError::CapabilityDenied)?;
        self.inner.default_agent(req.creation).map_err(DelegationError::AgentCreation)
    }
}

// api/types/sub_agent_spawn_request.rs — NEW
pub struct SubAgentSpawnRequest<'a> {
    pub parent_capabilities: CapabilityFlags,
    pub requested_capabilities: CapabilityFlags,
    pub parent_risk_ceiling: RiskLevel,
    pub requested_risk_ceiling: RiskLevel,
    pub creation: AgentCreationRequest<'a>,
}
```

**Where `parent_capabilities` comes from**: the composition root that assembled the parent agent already resolved a `ToolCapabilityModel` per skill to build each skill's `GovernedHandler` (ADR-046). That same composition root folds `CapabilityFlags::union`/`RiskLevel::max` once across the parent's registered skills to get the parent's own ceiling, and passes it into `SubAgentSpawnRequest` whenever the parent dispatches to a sub-agent it constructs. `DelegationSubsetPolicy` itself stays a pure two-bitset/two-enum comparison — it does not reflectively discover the parent's capabilities, exactly as `CapabilityGatePolicy` does not discover a tool's declared flags, it is handed them.

```
Composition root                         DelegatingAgentManager<M>
  parent_capabilities = union over            spawn_sub_agent(SubAgentSpawnRequest{ .. })
    parent's registered skills'                   │
    ToolCapabilityModel.capabilities()            ├─► DelegationSubsetPolicy.evaluate(DelegationRequest)
                                                   │      requested ⊆ parent?  risk ceiling ⊆ parent's?
                                                   ├─► on Err: DelegationError::CapabilityDenied (no agent built)
                                                   └─► on Ok: inner.default_agent(req.creation)
```

### Workspace layout (extends ADR-046's `edge-llm-tools`, no new crate)

```
domain/scm/domain/llm/tools/            (edge-llm-tools, still unbuilt per ADR-046)
├── api/
│   ├── traits/tool_capability_model.rs        (+ resource_from_input, default-provided)
│   ├── types/
│   │   ├── capability_flag.rs                 (unchanged, ADR-046)
│   │   ├── risk_level.rs                      (unchanged, ADR-046)
│   │   ├── capability_scope.rs                NEW
│   │   ├── path_pattern.rs                    NEW
│   │   ├── target_resource.rs                 NEW
│   │   ├── resource_extraction_request.rs     NEW
│   │   ├── resource_extraction_response.rs    NEW
│   │   ├── tool_invocation_request.rs         (made concrete: + scopes, + target_resource)
│   │   ├── capability_lookup_response.rs      (+ scopes: Vec<CapabilityScope>)
│   │   ├── delegation_request.rs              NEW
│   │   └── sub_agent_spawn_request.rs         NEW
│   └── errors/
│       ├── tool_capability_error.rs           (unchanged, ADR-046)
│       └── delegation_error.rs                NEW
├── core/
│   ├── governed_handler.rs                    (unchanged, ADR-046)
│   ├── capability_gate_policy.rs              (+ scoped-check branch, additive)
│   ├── risk_ceiling_policy.rs                 (unchanged, ADR-046)
│   ├── delegation_subset_policy.rs            NEW — impl Policy<Input=DelegationRequest>
│   └── delegating_agent_manager.rs            NEW — impl AgentManager + spawn_sub_agent
└── saf/
```

Depends on (unchanged from ADR-046, no new crate dependency added): `edge-domain-handler`, `edge-domain-policy`, `edge-security-runtime`, `edge-domain-observer`, `edge-llm-agent` (now also for `AgentManager`/`AgentCreationRequest`/`AgentCreationResponse`/`AgentError`, in addition to ADR-046's `ParameterDocumentation`/`SkillMetadata` reuse — same crate, no new edge added).

## What this ADR explicitly does NOT solve

- **Database row/table-level scoping (`DATABASE_READ`/`DATABASE_WRITE`)** — needs a query-shape-aware matcher (which table, which row predicate, which column set), not a glob over strings; a materially different `CapabilityScope` variant and a different `TargetResource` shape. Named as a follow-up, not designed here.
- **Network/URL scoping (`NETWORK_LOCAL`/`NETWORK_EXTERNAL`/`WEBHOOK_SEND`/`API_CALL`)** — needs host/port/path/method matching against an actual outbound request, plus a decision about redirects and DNS rebinding, out of scope for a single ADR alongside the two file-path cases. Named as a follow-up.
- **Scoping for `PROCESS_SPAWN`/`PROCESS_TERMINATE`/`MEMORY_ACCESS`/`SYSTEM_COMMAND`** — these do not have an obvious single-resource-identifier shape at all (a PID? a command line prefix? an address range?); left as all-or-nothing indefinitely unless a concrete need names a shape.
- **Recursive (multi-level) delegation depth limits.** A sub-agent that itself spawns a sub-sub-agent goes through the same `spawn_sub_agent` gate again with that sub-agent's own (already-narrowed) capabilities as the new `parent_capabilities` — the subset invariant composes correctly on its own (a chain of subsets is a subset) — but there is no explicit maximum delegation-chain-depth guard in this design, so a pathological or buggy agent could spawn an unbounded chain (each strictly narrower, but still unbounded in count). A depth counter/ceiling is a real, separate follow-up.
- **Revocation of already-granted sub-agent capabilities.** Delegation is a one-time, spawn-time check; nothing here lets a parent later narrow (or the system administratively revoke) a capability from a sub-agent that is already running. Would need a live capability-versioning/re-check mechanism, not designed here.
- **Path-pattern matching semantics hardening** (symlink escape, `..` traversal, case-insensitive filesystems, UNC paths on Windows) — `PathPattern`/glob matching is named as a string-pattern concept here; the actual matcher implementation must close these off, and is core/spi implementation work for whoever builds this, not a port-shape decision this ADR resolves.
- **Wiring any of this into a real composition root** (`edge-llm-runtime`, ADR-045) — this ADR (like ADR-046 before it) produces port shapes and policy impls; nothing here registers a `FileReadPaths`-scoped skill or calls `spawn_sub_agent` from a real dispatching skill. Separate follow-on work.
- **`resource_from_input`'s extraction correctness is entirely tool-author-supplied.** The default (`None`) is safe (fails closed per-scope, per above), but a tool author who declares `FileReadPaths` and then writes a buggy/incomplete `resource_from_input` (e.g. one that only handles one of several input shapes the skill actually accepts) creates a false sense of scoping without this ADR being able to detect it — no static or runtime check here proves an override is complete.

## Consequences

**What this enables**
- The two most concretely path-shaped capabilities (`FILE_READ`, `FILE_WRITE`) can be scoped to an allow-list of path patterns instead of staying all-or-nothing — a tool with `FileReadPaths(["/workspace/**"])` can be granted `FILE_READ` without also trusting it with `/etc/shadow`.
- A real, enforced safety invariant for agent-to-sub-agent delegation (`requested ⊆ parent` on both capabilities and risk ceiling), closing ADR-046's named gap with an actual `Policy` impl and an actual construction-time gate, not just a stated intention.
- Zero changes to `edge-llm-agent`'s `Agent`/`AgentManager`/`Skill` traits or to `AgentCreationRequest` — both extensions live entirely inside `edge-llm-tools`, consistent with ADR-046's own precedent of keeping cross-cutting concerns out of the domain trait surface.
- `CapabilityGatePolicy`'s existing bit-only behavior is preserved byte-for-byte for the 10 unscoped capabilities and for every tool that doesn't opt into `CapabilityScope` — this is a pure extension, not a behavior change for anything not explicitly scoped.

**What this requires**
- `ToolCapabilityModel` gains one new default-provided trait method (`resource_from_input`) — additive, does not force existing (hypothetical, none built yet) implementors to change.
- `CapabilityLookupResponse` and `ToolInvocationRequest` each gain one new field (`scopes`, and `scopes`/`target_resource` respectively) — since `edge-llm-tools` is unbuilt, this is a design amendment, not a breaking change to running code; it does mean anyone who started implementing directly off ADR-046's prose alone (no code exists) should build against this ADR's concrete field list instead.
- A new decorator type (`DelegatingAgentManager<M>`) and a new, non-trait method (`spawn_sub_agent`) that composition roots must call instead of `AgentManager::default_agent` directly whenever an agent is being constructed as another agent's delegate — calling `default_agent` directly still works and stays fully unguarded, exactly the same opt-in posture ADR-046 already accepted for `GovernedHandler` ("an ungoverned registration is still possible and won't be flagged by tooling").
- Composition roots must actually compute and track `parent_capabilities`/`parent_risk_ceiling` per agent (folding across its registered skills' `ToolCapabilityModel`s) to pass into `spawn_sub_agent` — new bookkeeping, not automatic.

## Alternatives Considered

**Extend `AgentManager` itself with a `spawn_sub_agent` default-provided method (mirroring the existing `conversation_loop`/`agent_metadata_builder` growable-trait pattern in `agent_manager.rs`)**
Rejected. `AgentManager` lives in `edge-llm-agent`, which `edge-llm-tools` depends on, not the reverse; a `spawn_sub_agent` signature that takes `CapabilityFlags`/`RiskLevel` and returns `DelegationError` (a `PolicyError`-wrapping type) would force `edge-llm-agent` to depend on `edge-llm-tools`'s vocabulary, inverting the one-directional dependency ADR-046 established. The growable-trait pattern is real and reusable in principle, but not here — it only works when the new method's types already live in the trait's own crate.

**Widen `AgentCreationRequest` with optional `parent`/`delegated_capabilities` fields instead of a separate `SubAgentSpawnRequest`**
Rejected. `AgentCreationRequest` is a general-purpose "build any agent" request, used identically whether or not the agent being built is anyone's delegate; bolting delegation-specific fields onto it conflates "how do I build an agent" with "how do I build an agent as a bounded delegate of another," and (same dependency issue as above) `CapabilityFlags`/`RiskLevel` would have to move into `edge-llm-agent` for the field types to be nameable there at all. A wrapping request type in `edge-llm-tools` keeps `AgentCreationRequest` exactly as ADR-046/prior work left it.

**Make the delegation check itself a `Policy<Input = ToolInvocationRequest>` entry in the same `CompositePolicy` `GovernedHandler` already evaluates, rather than a separate gate at spawn time**
Rejected — wrong lifecycle stage, not just wrong port. `GovernedHandler` gates *invocation* of an already-constructed skill; delegation must be checked *before* a sub-agent (and its skills) exist at all, otherwise an over-privileged sub-agent could be fully constructed and only caught the first time one of its skills is invoked — by which point the sub-agent object itself, and anything it did before its first gated call (e.g. reading its own configuration, or a skill call that happens to need no capability at all), already ran unchecked. A construction-time gate (`spawn_sub_agent`) is the only point that can refuse to bring the over-privileged agent into existence in the first place.

**Widen `PolicyError` (or invent a `DelegationPolicyError`) instead of a distinct `DelegationError` enum**
Rejected, same reasoning ADR-072 already used for moderation: `CompositePolicy<I>` requires every composed `Policy<Input=I>` to share one concrete error type, and `DelegationSubsetPolicy`'s `evaluate` correctly returns plain `PolicyError` to stay composable with any future `Policy<Input=DelegationRequest>` added later. `DelegationError` is not `Policy::evaluate`'s return type — it is `spawn_sub_agent`'s return type, one layer up, where it must also represent `AgentCreation` failure (a distinct `AgentError`, nothing to do with policy at all); collapsing those two failure modes into a single type there (rather than into `PolicyError` itself) keeps `PolicyError` untouched for every existing/future policy composition.

**Fail-open when `resource_from_input` returns `None` for a scoped capability (fall back to coarse-grained bit check)**
Rejected. A tool author declaring `FileReadPaths` is making an explicit, narrower claim than "I need `FILE_READ`, full stop"; silently falling back to the wider grant on missing extraction data would mean a scoping declaration only ever narrows behavior when everything goes right, and reverts to trusting the tool fully the moment extraction fails or is unimplemented — the opposite of what declaring a scope is meant to guarantee.

## Tracking

- Extends: ADR-046's `edge-llm-tools` (`domain/scm/domain/llm/tools/`, still not implemented) — this ADR's field/method additions must land in the same first implementation pass, not as a later patch, since `CapabilityLookupResponse`/`ToolInvocationRequest` are amended here before either has shipped
- Follow-up (separate ADR): database row/table-level scoping shape for `DATABASE_READ`/`DATABASE_WRITE`
- Follow-up (separate ADR): network/URL scoping shape for `NETWORK_LOCAL`/`NETWORK_EXTERNAL`/`WEBHOOK_SEND`/`API_CALL`
- Follow-up (not blocking): delegation-chain depth ceiling to bound recursive sub-agent-of-sub-agent spawning
- Follow-up (not blocking): live capability revocation/re-check for already-running sub-agents
- Follow-up (implementation-level, not a port decision): hardened `PathPattern` matcher (symlink/`..`/case/UNC handling)
- Follow-up (separate ADR/issue, mirrors ADR-046's own tracking): wire `DelegatingAgentManager`/`spawn_sub_agent` into a real composition root (`edge-llm-runtime`, ADR-045) and require it (not `AgentManager::default_agent` directly) for any code path building a sub-agent
- Not blocking this ADR: `resource_from_input` override completeness is not statically or runtime checked; a tool author's own responsibility, noted as a real limitation, not solved here
