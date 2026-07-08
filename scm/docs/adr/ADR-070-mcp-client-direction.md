# ADR-070: `edge-llm-mcp` MCP Client Direction — `McpToolClient`/`RemoteMcpSkill` for Calling Arbitrary External MCP Servers

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-030 (Agent), ADR-032 (LLM Agent), ADR-046 (`edge-llm-tools` Governance — capability gating for external calls), ADR-047 (`edge-runtime-mcp`/`edge-llm-mcp` MCP server direction, sibling), [edge ADR-003](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-003-egress-process-runner.md) (egress/process — `ProcessRunner`, investigated and ruled out below as a stdio-transport foundation)
**GitHub Issues:** TBD

---

## Context

ADR-047 designed `edge-runtime-mcp` (wire-protocol contracts) and `edge-llm-mcp` (LLM-domain adapter) in full for **both** MCP directions — server (`McpToolIngress`/`HandlerRegistryMcpIngress`) and client (`McpToolEgress`/`RemoteMcpSkill`) — but shipped only the server half in v1, deferring the client half with a specific, named blocker: "it requires the actual stdio/SSE JSON-RPC transport implementation... to exist first — that transport crate is out of scope here... and blocks `McpToolEgress` from having a real, non-mock implementation" (ADR-047 §"Server vs. client scope for v1").

Per explicit direction on this audit series — nothing stays merely deferred, every deferred item gets its own dedicated follow-up ADR — this ADR promotes that client direction out of ADR-047's "designed, not shipped" limbo into a concrete, buildable design, and it does the thing ADR-047 itself did not do: **it goes and checks whether the stated blocker is actually still true.**

It is only *half* true.

### Investigating the blocker: does a reusable stdio or SSE transport already exist?

**SSE — yes, and it is directly reusable today.** `transport/egress/http/scm/transport/main/src/api/traits/http/http_stream.rs:20-31` (already cited by ADR-048, which built `AnthropicCompleter`'s real streaming path on top of it) defines:

```rust
pub trait HttpStream: Send + Sync {
    fn subscribe_sse(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<SseStream>>;
    fn connect_websocket(&self, url: &str) -> BoxFuture<'_, HttpEgressResult<WsChannel>>;
}
```

where `SseStream = Pin<Box<dyn Stream<Item = Result<SseEvent, HttpEgressError>> + Send>>` and `SseEvent { event: Option<String>, data: String, id: Option<String> }` already parses generic `text/event-stream` framing (`event:`/`data:`/`id:` fields) — the exact chunked-transfer line-framing work that would otherwise have to be built from scratch. This is the *same* infrastructure ADR-048 used to prove real incremental completion streaming; it is equally usable as the transport underneath an SSE-based MCP server connection. What remains missing for MCP specifically is purely protocol-level, not transport-level: constructing a JSON-RPC 2.0 envelope (`{"jsonrpc":"2.0","id":...,"method":"tools/call","params":{...}}`), sending it, and correlating the matching response event out of the `SseStream` by request id — exactly the shape of gap ADR-048 closed for Anthropic's SSE event vocabulary, not a new transport primitive.

**Stdio — no, and `ProcessRunner` (ADR-003) does not close the gap.** `transport/egress/subprocess` (`edge-egress-subprocess`, confirmed present in the workspace as a submodule with structural-audit history matching ADR-003's `egress/process/main/features/process` layout) provides exactly what ADR-003 specified:

```rust
pub trait ProcessRunner: Send + Sync + 'static {
    async fn run(&self, args: ProcessArgs) -> ProcessResult;
}
```

This is a **one-shot, run-to-completion** contract: spawn, race `child.wait_with_output()` against a deadline, capture combined stdout+stderr up to a byte cap, return once the child has *exited*. MCP's stdio transport needs the opposite shape: a single child process that stays running indefinitely for the lifetime of the connection, while the client writes one JSON-RPC request line to its stdin and reads one JSON-RPC response line from its stdout per call — `tools/list` once, then `tools/call` an arbitrary number of times, in whatever order and timing the agent needs, all against the *same* live process and its accumulated server-side session state. There is no way to get that behavior out of `ProcessRunner::run` without either (a) spawning a brand-new server subprocess per JSON-RPC call — which breaks any MCP server that holds session/connection state and adds a full process-spawn latency to every tool invocation, or (b) not using `ProcessRunner` at all. `ProcessRunner`'s allow-list/timeout/byte-cap policy model is aimed at *deterministic, single-shot* command execution (ADR-003's own consumers are ADS's command server and llmboot's `run_command` executor step — both single-invocation use cases); a long-lived bidirectional pipe with incremental, multiply-reentrant read/write is a materially different concern, not a variant of the same one. **ADR-047's stdio blocker stands**, confirmed by direct inspection rather than assumed.

Net finding: the client direction is **not uniformly blocked**. An SSE/HTTP-reachable MCP server can get a real, non-mock `McpToolEgress` today. A stdio-spawned MCP server (the more common local-tool-server shape, and the one llmboot's bespoke ADS client already uses) still cannot, until a genuinely new long-lived-subprocess-pipe primitive is built — which is *not* `ProcessRunner` and is *not* scoped by this ADR either, for the same reason ADR-047 didn't scope its transport crate: it is real, separate, follow-on work, not a design question this ADR needs to resolve to proceed.

### Why this matters beyond ADR-047's scope: the client direction is a trust boundary the server direction never was

ADR-047's server direction exposes **your own** already-registered `Skill`/`Handler` instances to an external caller — the caller is the untrusted party, and `GovernedHandler` (ADR-046) already gates that inbound surface for free, as ADR-047 itself noted. The client direction inverts this: an `Agent` reaching out to `RemoteMcpSkill` is now the one placing a call to **somebody else's** process, over a network or a spawned subprocess, sending it live call arguments (which may contain user data, conversation content, or file contents constructed by the LLM) and then feeding whatever that remote process returns back into the agent's context. This is a genuine egress/trust boundary — the MCP-protocol analogue of ADR-003's `allow_commands` allow-list, ADR-046's `NETWORK_EXTERNAL`/`API_CALL` capability flags, and ADR-048's credential-boundary discipline, all at once. ADR-047 explicitly did not need to address this for the server direction; this ADR must, and does (see Decision).

## Decision

Reuse `edge-runtime-mcp`'s already-designed client-side contract unchanged — `McpToolEgress`, `McpListRemoteToolsRequest`/`Response`, `McpCallRemoteToolRequest`/`Response`, `McpToolDefinition`, `McpToolError` (ADR-047 §Shape) — because the wire shape of "list a remote server's tools" / "call one of them" is exactly as protocol-agnostic-of-domain in the client direction as `McpToolIngress`'s shape already is in the server direction. No parallel JSON-RPC vocabulary is invented here.

```rust
// edge-runtime-mcp/api — unchanged from ADR-047, reused verbatim
pub trait McpToolEgress: Send + Sync {
    fn list_remote_tools(&self, req: McpListRemoteToolsRequest) -> Result<McpListRemoteToolsResponse, McpToolError>;
    fn call_remote_tool(&self, req: McpCallRemoteToolRequest) -> Result<McpCallRemoteToolResponse, McpToolError>;
}

pub struct McpListRemoteToolsRequest;   // egress is bound to one already-known server per instance —
                                        // no remote-endpoint parameter needed on the call itself,
                                        // matching ADR-047's stated one-server-at-a-time granularity.
pub struct McpListRemoteToolsResponse { pub tools: Vec<McpToolDefinition> }

pub struct McpCallRemoteToolRequest {
    pub tool_name: String,
    pub arguments: serde_json::Value,
}
pub struct McpCallRemoteToolResponse {
    pub content: serde_json::Value,
    pub is_error: bool,
}
```

`McpToolError::Denied` here means the **remote server** refused the call (their-side denial, e.g. their own tool-level auth); it is distinct from a governance denial on **our** side, which surfaces as `HandlerError` wrapping `PolicyError` from `GovernedHandler` (see below) — two different trust boundaries, not one, and they must not be conflated into a single error variant.

Three new things are needed to make this real, in three different crates, at three different coupling levels — following the same tier discipline ADR-047 used to split `edge-runtime-mcp` from `edge-llm-mcp`, and ADR-048 used to keep vendor HTTP/SSE mechanics out of the domain crate:

1. **A new transport crate, `swe-edge-egress-mcp`** (`egress/mcp/`, analogous to `swe-edge-egress-http`) — owns the actual JSON-RPC 2.0 envelope construction/parsing, request-id correlation, and the two concrete `McpToolEgress` backings. This is where the investigation above pays off directly: one backing is real today, one is still blocked.
2. **`edge-llm-mcp` gains `RemoteMcpSkill` for real** (ADR-047 already designed its shape; this ADR ships it) plus a new `McpToolClient` facade that makes governance structurally mandatory for the client direction specifically (not merely opt-in, unlike the server direction).
3. **No changes to `edge-runtime-mcp`, `edge-domain-handler`, `edge-security-runtime`, `edge-domain-observer`, `edge-llm-agent`, or ADR-046's `GovernedHandler`/`CompositePolicy`/`Policy` traits.** Every new capability is composed from existing ports, exactly the discipline ADR-046 used against ADR-036.

### Shape / workspace layout

#### `swe-edge-egress-mcp` (new transport crate — owns JSON-RPC mechanics + concrete `McpToolEgress` backings)

```
egress/mcp/scm/
└── main/src/
    ├── core/
    │   └── jsonrpc/
    │       ├── envelope.rs            # {"jsonrpc":"2.0","id","method","params"} construct/parse, shared by both backings
    │       └── correlation.rs         # request-id → pending-response matching
    └── spi/
        ├── http_sse_mcp_tool_egress.rs   # REAL, v1-shippable
        └── stdio_mcp_tool_egress.rs      # DESIGNED, NOT shipped — see "does NOT solve"
```

```rust
/// Real, non-mock McpToolEgress backed by transport/egress/http's HttpStream::subscribe_sse.
/// Constructed with the remote server's URL — one instance per known server, per
/// ADR-047's one-server-at-a-time scope.
pub(crate) struct HttpSseMcpToolEgress {
    http_stream: Arc<dyn HttpStream>,
    server_url: String,
}

impl McpToolEgress for HttpSseMcpToolEgress {
    fn list_remote_tools(&self, req: McpListRemoteToolsRequest) -> Result<McpListRemoteToolsResponse, McpToolError> {
        // build {"method":"tools/list"} JSON-RPC envelope, POST/subscribe via subscribe_sse(&self.server_url),
        // correlate the matching SseEvent by request id, parse its `data` field as the JSON-RPC result.
    }
    fn call_remote_tool(&self, req: McpCallRemoteToolRequest) -> Result<McpCallRemoteToolResponse, McpToolError> {
        // same envelope/correlation pattern for {"method":"tools/call","params":{"name":..,"arguments":..}}
    }
}

/// Designed, not shipped. See "What this ADR explicitly does NOT solve."
pub(crate) struct StdioMcpToolEgress { /* blocked on a long-lived subprocess-pipe primitive */ }
```

Depends on: `edge-runtime-mcp` (`McpToolEgress`, wire types, `McpToolError`), `swe-edge-egress-http` (`HttpStream`, `SseStream`, `SseEvent` — already sufficient, zero changes needed, same crate ADR-048 already depends on).

#### `edge-llm-mcp` (domain adapter — client direction shipped for real)

```
domain/scm/domain/llm/mcp/
├── api/
│   ├── types/
│   │   ├── remote_tool_skill_{request,response}.rs        # unchanged, ADR-047
│   │   └── discover_remote_skills_{request,response}.rs   # NEW, this ADR
│   └── errors/mcp_bridge_error.rs                          # extended, this ADR
├── core/
│   ├── remote_mcp_skill.rs      # Skill/Handler impl wrapping Arc<dyn McpToolEgress> + tool name — shipped for real
│   └── mcp_tool_client.rs       # NEW: McpToolClient — mandatory-governance discovery facade
└── saf/
    └── mcp_tool_client_svc.rs   # NEW: re-export/factory
```

```rust
/// Client direction, shipped: wraps one remote MCP tool as an ordinary Skill/Handler.
/// Unchanged from ADR-047's design — now backed by a real HttpSseMcpToolEgress.
pub(crate) struct RemoteMcpSkill {
    egress: Arc<dyn McpToolEgress>,
    tool_name: String,
}

impl Skill for RemoteMcpSkill { /* name/description/schema sourced from the McpToolDefinition
                                   discovered at McpToolClient::discover_skills time */ }

#[async_trait]
impl Handler for RemoteMcpSkill {
    type Request = serde_json::Value;
    type Response = serde_json::Value;

    async fn execute(&self, req: ExecutionRequest<'_, Self::Request>) -> Result<Self::Response, HandlerError> {
        self.egress
            .call_remote_tool(McpCallRemoteToolRequest { tool_name: self.tool_name.clone(), arguments: req.req })
            .map(|resp| resp.content)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))
    }
}

/// NEW: the only sanctioned way to obtain skills from a remote MCP server.
/// Structurally forces capability declaration — there is no path to a bare,
/// ungoverned RemoteMcpSkill through this API. RemoteMcpSkill's constructor
/// stays pub(crate); this is the sole public entry point.
pub struct McpToolClient {
    egress: Arc<dyn McpToolEgress>,
}

impl McpToolClient {
    pub fn new(egress: Arc<dyn McpToolEgress>) -> Self { Self { egress } }

    /// Lists every tool the remote server exposes and wraps each as
    /// GovernedHandler<RemoteMcpSkill>, gated by one caller-supplied capability
    /// declaration applied uniformly to every tool from this server — see
    /// Decision, "Why one capability declaration per server, not per tool."
    pub fn discover_skills(
        &self,
        req: DiscoverRemoteSkillsRequest,
    ) -> Result<DiscoverRemoteSkillsResponse, McpBridgeError> {
        if req.risk_level < RiskLevel::Medium {
            return Err(McpBridgeError::InsufficientRiskFloor {
                declared: req.risk_level,
                minimum_required: RiskLevel::Medium,
            });
        }
        if !req.capabilities.contains(CapabilityFlags::NETWORK_EXTERNAL) {
            return Err(McpBridgeError::MissingRequiredCapability(CapabilityFlags::NETWORK_EXTERNAL));
        }
        let tools = self.egress.list_remote_tools(McpListRemoteToolsRequest)
            .map_err(McpBridgeError::from)?
            .tools;
        let skills = tools.into_iter().map(|tool| {
            let skill = RemoteMcpSkill { egress: self.egress.clone(), tool_name: tool.name.clone() };
            Box::new(GovernedHandler::new(skill, req.capability_model.clone(), req.policy.clone())) as Box<dyn Skill>
        }).collect();
        Ok(DiscoverRemoteSkillsResponse { skills })
    }
}
```

```rust
pub struct DiscoverRemoteSkillsRequest {
    pub capabilities: CapabilityFlags,               // must include NETWORK_EXTERNAL — enforced, not advisory
    pub risk_level: RiskLevel,                        // floor-checked at Medium — enforced, not advisory
    pub capability_model: Arc<dyn ToolCapabilityModel>,
    pub policy: Arc<dyn Policy<Input = ToolInvocationRequest>>,
}
pub struct DiscoverRemoteSkillsResponse {
    pub skills: Vec<Box<dyn Skill>>,   // every entry already GovernedHandler-wrapped
}

pub enum McpBridgeError {
    Transport(McpToolError),
    InsufficientRiskFloor { declared: RiskLevel, minimum_required: RiskLevel },
    MissingRequiredCapability(CapabilityFlags),
}
```

Depends on: `edge-runtime-mcp` (`McpToolEgress`, wire types), `edge-llm-agent` (`Skill`, `SkillMetadata`), `edge-domain-handler` (`Handler`, `HandlerError`, `ExecutionRequest`), `edge-llm-tools` (`GovernedHandler`, `ToolCapabilityModel`, `CapabilityFlags`, `RiskLevel`, `ToolInvocationRequest` — ADR-046), `edge-domain-policy` (`Policy`, `PolicyError` — via ADR-046, not a new direct dependency choice, just the transitive shape `GovernedHandler` already requires).

### Why one capability declaration per server, not per tool

`McpToolDefinition { name, description, input_schema }` (ADR-047) is everything MCP's `tools/list` response tells you about a remote tool. It carries **no capability or risk vocabulary at all** — a tool named `"read_file"` might genuinely only read a file, or it might also make a network call, spawn a process, or write to disk; the JSON-RPC schema is a documentation-quality, not an behavior-verifying, contract. `edge` has no way to validate that a remote tool's declared name/description/schema matches its actual side effects — that is a fundamentally different problem from ADR-046's original design, which assumed a `Skill` author declares its own capabilities honestly because the author and the code are the same trust domain. Here they are not.

Given that, `McpToolClient::discover_skills` applies **one conservative capability declaration to every tool discovered from a given server**, supplied by whoever registers that server's `McpToolEgress` in the composition root — not a (falsely precise) per-tool declaration inferred from the remote server's own self-reported metadata. `NETWORK_EXTERNAL` is structurally required (enforced by the `MissingRequiredCapability` check above, not merely documented as a convention) because every call through `RemoteMcpSkill` is, definitionally, an external network or subprocess call. `RiskLevel::Medium` is enforced as a floor for the same reason ADR-003's `allow_commands` defaults to "empty list → all commands blocked": declaring "no risk" for a call to a third party's arbitrary code is not a modeling error a composition root should be able to make by omission.

This makes governance for the client direction **structurally mandatory**, not opt-in — a deliberate strengthening of ADR-046's original stance ("an ungoverned registration is still possible and won't be flagged by tooling," ADR-046 §Consequences), scoped specifically to the client direction, where the trust boundary is real (see Context) and was not a design gap for the server direction ADR-046/ADR-047 already covered.

## What this ADR explicitly does NOT solve

- **The stdio MCP transport.** `StdioMcpToolEgress` is designed (see Shape) but not shipped. It needs a long-lived, bidirectional, incrementally-read/written subprocess pipe — spawn once, write many JSON-RPC request lines to stdin, read many response/notification lines from stdout, across the connection's full lifetime — which is a different trait shape from `ProcessRunner::run`'s one-shot spawn-wait-capture-exit contract (see Context investigation). Building that primitive (name TBD, e.g. a second trait alongside `ProcessRunner` in `swe-edge-egress-subprocess`, or a wholly separate crate) is real, separate follow-on work, explicitly out of scope here, for the same reason ADR-047 didn't scope its own transport crate: it's a prerequisite, not a design question this ADR needs answered to proceed with the SSE half.
- **Verifying a remote tool's actual behavior against its declared schema/description.** `McpToolClient::discover_skills`'s capability gate controls whether the *call* is allowed to happen at all (the invocation surface); it says nothing about whether the remote tool's response is safe to feed back into the agent's context (prompt-injection-via-tool-result), nor whether the remote tool actually does only what its name/description claim. That is a content-inspection problem, not a capability-gating one, and belongs with ADR-053 (guardrails/content moderation) if and when a consumer needs it against MCP tool results specifically.
- **MCP capability-negotiation handshake, notifications, "resources"/"prompts" primitives.** Same scope line ADR-047 already drew for the server direction; unchanged here for the client direction.
- **A registry/directory of known external MCP servers.** `McpToolClient` is constructed against one already-known `McpToolEgress` (one server) at a time, same granularity ADR-047 specified.
- **Per-tool (as opposed to per-server) capability declarations.** See Decision — deliberately conservative and server-scoped, not a gap to be filled by a smarter parser of `McpToolDefinition.description`; MCP's own wire format has no per-tool capability vocabulary to parse.
- **Replacing llmboot's existing bespoke ADS-only MCP client.** Unaffected, exactly as ADR-047 already stated for the server direction; this ADR generalizes the *pattern* llmboot's client already proves is useful, it does not migrate llmboot itself.
- **Credential/auth handling for the remote MCP server's own endpoint** (e.g. a bearer token the SSE server itself requires). `HttpSseMcpToolEgress` takes a bare `server_url`; wiring `edge-security-runtime-credential` (ADR-048's pattern) through to an authenticated remote MCP endpoint is a mechanical follow-up using the same credential-resolution mechanism ADR-048 already proved, not a new design question, but not built here.

## Consequences

**What this enables**
- A real, non-mock `McpToolEgress` implementation exists for SSE/HTTP-reachable MCP servers today — `edge-plugin-a2a`-style agents or any `edge-llm-agent` `Agent` can call a third-party MCP tool server over SSE without edge growing a bespoke per-server adapter, closing half of ADR-047's stated blocker by direct investigation rather than assumption.
- llmboot's bespoke, ADS-only MCP client now has a named, concrete, buildable generalization target for the SSE case specifically (the stdio case — the one llmboot's ADS integration actually uses today — remains blocked, honestly reported, not silently implied to be solved).
- Governance for the client direction is structurally mandatory, not opt-in-and-easy-to-forget: `McpToolClient::discover_skills` is the only public path to a `RemoteMcpSkill`, and it enforces `NETWORK_EXTERNAL` + a `RiskLevel::Medium` floor before returning anything. This closes the one governance gap ADR-046/ADR-047 left unaddressed because it didn't yet exist as a real code path.
- `ProcessRunner` (ADR-003) is confirmed, by direct code-shape comparison, to not be silently reusable for this — preventing a future implementer from taking a shortcut that would either serialize every MCP tool call through a fresh subprocess spawn or silently break any stateful MCP server.

**What this requires**
- New crate `swe-edge-egress-mcp` (`egress/mcp/`), depending on `edge-runtime-mcp` and `swe-edge-egress-http` — zero changes to either.
- `edge-llm-mcp` gains `RemoteMcpSkill` (shipped, was designed-only in ADR-047) and `McpToolClient`/`DiscoverRemoteSkillsRequest`/`Response`/`McpBridgeError` (new), depending additionally on `edge-llm-tools` (ADR-046) for `GovernedHandler`/`ToolCapabilityModel`/`CapabilityFlags`/`RiskLevel`.
- No changes to `edge-runtime-mcp`'s `McpToolEgress` trait or wire types, `edge-domain-handler`, `edge-security-runtime`, `edge-domain-observer`, `edge-llm-agent`, or ADR-046's `GovernedHandler`/`Policy`/`CompositePolicy`.
- A still-open, separately-tracked prerequisite (stdio pipe primitive) before `StdioMcpToolEgress` can ship — named and scoped here, not silently deferred without a track record, per this ADR's own mandate to close out ADR-047's deferral honestly rather than re-defer it a second time.

## Alternatives Considered

**Reuse `ProcessRunner` directly for stdio MCP transport by spawning one subprocess per JSON-RPC call**
Rejected. Breaks any MCP server with connection-scoped session state (the MCP handshake, tool-list caching, or any stateful tool implementation on the remote side would be reset every call), and adds a full process-spawn latency to every single tool invocation instead of once per session. This is not a performance nitpick — it changes the correctness semantics of talking to a real MCP server, which assumes a persistent connection.

**Ship both `HttpSseMcpToolEgress` and `StdioMcpToolEgress` together, blocking this ADR until the stdio pipe primitive exists**
Rejected. The SSE backing is genuinely unblocked today (see Context investigation) and independently useful; gating it on the harder, unrelated stdio primitive repeats exactly the scope-creep ADR-048 avoided by shipping one vendor fully real instead of two partially real. Ship what's real now, track the rest honestly (Tracking).

**Make capability declaration per-tool (parse `McpToolDefinition.description` heuristically for capability hints)**
Rejected. There is no reliable way to infer `PROCESS_SPAWN` vs `FILE_WRITE` vs `DATABASE_WRITE` from a free-text tool description without trusting the remote server's own honesty about what its tool does — which is exactly the trust boundary this ADR exists to gate, not lean on. A conservative, server-scoped, structurally-enforced floor is honest about what `edge` can actually verify (nothing about the remote tool's real behavior) versus what it can enforce (that a capability/risk declaration exists and meets a floor before the call is even attempted).

**Leave the client direction opt-in-governed, consistent with ADR-046's default stance for the server direction**
Rejected specifically for this direction. ADR-046 left server-direction governance opt-in because an ungoverned *local* skill is still a known, first-party piece of code running in the same trust domain as everything else already registered. An ungoverned `RemoteMcpSkill` is a live network/subprocess call to a third party outside `edge`'s control entirely — the asymmetry in Context justifies a stricter default here without needing to change ADR-046's trait or its stance for the server direction.

**Build a single combined `McpToolClientAndServer` facade instead of keeping `McpToolClient` (this ADR) separate from `HandlerRegistryMcpIngress` (ADR-047)**
Rejected. Same reasoning ADR-047 already used to split `McpToolIngress`/`McpToolEgress`: disjoint call shapes, disjoint failure modes, and now, disjoint governance defaults (opt-in for server, mandatory for client) — a combined facade would either force the stricter default onto the server direction unnecessarily or silently weaken it for the client direction. Keeping them separate lets each direction's governance posture match its actual trust boundary.

## Tracking

- New crate: `swe-edge-egress-mcp` (`egress/mcp/`) — `HttpSseMcpToolEgress` (real, v1), `StdioMcpToolEgress` (designed, blocked), shared JSON-RPC envelope/correlation core
- `edge-llm-mcp` additions: `RemoteMcpSkill` (shipped for real, was designed-only in ADR-047), `McpToolClient`, `DiscoverRemoteSkillsRequest`/`Response`, extended `McpBridgeError`
- Follow-up, separate ADR/issue: the long-lived subprocess-pipe primitive for `StdioMcpToolEgress` — explicitly not `ProcessRunner`; investigate whether it belongs in `swe-edge-egress-subprocess` as a second trait or a new crate
- Follow-up: wire `McpToolClient`-discovered skills into `edge-llm-runtime` (ADR-045) registration alongside `HandlerRegistryMcpIngress` (ADR-047's own tracked follow-up), so both MCP directions register through the same composition root
- Follow-up: credential resolution for authenticated remote MCP servers, reusing `edge-security-runtime-credential` per ADR-048's already-proven pattern
- Follow-up: once a stdio pipe primitive exists, revisit whether `StdioMcpToolEgress` should get the same structurally-mandatory capability floor as `HttpSseMcpToolEgress` (expected: yes, unchanged design, mechanical wiring only)
- Not addressed here, tracked as a distinct future concern: content-level inspection of remote MCP tool results before they re-enter agent context (candidate home: ADR-053 guardrails, if a concrete need arises)
- Not tracked here, unaffected: ADS's `swe_agentdevstudio_mcp_server_command`, llmboot's `tool_invoke` executor (ADR-003's existing scope), ADR-047's server direction (`HandlerRegistryMcpIngress`, already tracked there)
