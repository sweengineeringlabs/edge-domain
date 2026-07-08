# ADR-047: `edge-runtime-mcp` + `edge-llm-mcp` — MCP (Model Context Protocol) Port

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-030 (Agent), ADR-032 (LLM Agent), ADR-045 (`edge-llm-runtime` Composition Root), ADR-046 (`edge-llm-tools` Governance), [edge ADR-047](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-047-edge-runtime-primitive-family.md) (`edge-runtime-*` primitive family), [edge ADR-003](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-003-egress-process-runner.md) (egress/process — mentions ADS MCP server)
**GitHub Issues:** TBD

---

## Context

The 2026-07-08 LLM-landscape audit found that MCP (Model Context Protocol — JSON-RPC 2.0 tool-call protocol over stdio/SSE/HTTP transports) has **no port anywhere inside `edge`**, despite MCP already existing in this ecosystem, just outside it:

- `docs/3-architecture/adr/ADR-003-egress-process-runner.md:19-23` documents that **agent-development-studio (ADS)** owns `swe_agentdevstudio_mcp_server_command`'s `CommandServer` — an MCP *server* that exposes subprocess execution as a tool call, with its own allow-list and per-call timeout, "every policy extension requires changing the server binary."
- The same file, lines 24-29, documents that **llmboot** (the actual agent SDK; see also ADR-031 in the `edge-domain-agent` memory line, not in this repo) is an MCP *client*: its `tool_invoke` executor "routes `run_command` steps through the ADS MCP command server via a JSON-RPC round-trip (MCP client → stdio transport → subprocess → response)," and ADR-003 calls this out as "the full protocol overhead... for a local subprocess call that could be in-process" for the one case it fixes (process spawning). ADR-003's own Alternatives Considered (lines 240-243) reaffirms the split: "Expose as an MCP tool... Rejected for deterministic executor steps... The MCP server remains the right path for LLM-dispatched tool calls" — i.e., MCP is confirmed as the correct transport for genuinely LLM-dispatched calls, just not for the deterministic process-spawn case ADR-003 itself solves.
- `docs/3-architecture/adr/ADR-047-edge-runtime-primitive-family.md` (top-level edge repo, read in full) proposes ten `edge-runtime-*` contract crates — http, grpc, messaging, queue, db, cache, lb, cloud, cli, agent — mirroring `edge-domain-*` as "contracts only... suitable as a dependency for both plugins... and transport crates." No `-mcp` appears in that list, despite MCP being arguably the most LLM-native transport of the set.
- `domain/scm/domain/llm/agents/main/src/api/traits/skill.rs` (read in full) shows `Skill: Handler` already carries `name()` → `SkillNameResponse`, `description()` → `SkillDescriptionResponse`, `parameters()`/`parameter_documentation()`, `input_schema()`/`output_schema()`, and `metadata()` → `SkillMetadataLookupResponse{ metadata: SkillMetadata }`. `SkillMetadata` (`agents/main/src/api/types/skill_metadata.rs:5-18`) is `{ name, description, input_schema: Option<String>, output_schema: Option<String>, async_execution, long_running }` — a JSON-Schema-bearing name/description/schema triple, which is exactly MCP's `Tool { name, description, inputSchema }` shape. `ParameterDocumentation` (`parameter_documentation.rs:5-20`) — `{ name, description, param_type, required, default, examples, validation_rules }` — maps directly onto MCP `inputSchema.properties[*]`. This is a real, load-bearing coincidence, not an analogy stretched to fit: no new "tool descriptor" vocabulary needs inventing on the server side.
- `domain/scm/docs/adr/ADR-046-edge-llm-tools-governance.md`'s `GovernedHandler<H>` (any `Handler`-wrapping decorator that gates on `CompositePolicy<ToolInvocationRequest>` before delegating to `self.inner.execute(...)`) already applies to any handler exposed through a future MCP-ingress adapter with zero extra code, because an MCP-exposed tool is still just a `Handler` underneath — this is a design validation of ADR-046, not new work.
- `edge-runtime-http` (`runtime/http/scm/`, read in full via directory listing) is the closest existing precedent for shape: `api/server/traits/http_server.rs:18` defines `HttpServer` as a small trait wrapping `Arc<dyn HttpIngress>` (the actual `HttpIngress` trait itself still lives in the external `swe-edge-ingress-http` crate today, not yet migrated per the top-level ADR-047's Phase 1 — confirmed by `use swe_edge_ingress_http::HttpIngress;` at `http_server.rs:7`). This is useful precedent for *layering* (protocol contract vs. LLM-domain glue) even though the top-level migration itself isn't finished yet.
- `domain-handler`'s `Handler` trait (`domain/scm/domain-handler/main/src/api/handler/traits/handler.rs:13-39`) is `async fn execute(&self, req: ExecutionRequest<'_, Self::Request>) -> Result<Self::Response, HandlerError>`, and `HandlerContext<'a>` (`domain/scm/domain-handler/main/src/api/handler/types/handler_context.rs:12-19`) carries `security: &SecurityContext`, `commands: &dyn CommandBus`, `observer: &dyn ObserverContext` — the same three seams ADR-046 already reuses for tool governance. Any MCP adapter that ends at a `Handler::execute` call inherits all three for free.

No `domain-policy`-shaped MCP-specific governance concept is needed here — ADR-046 already built it, and it composes for free (see Decision).

## Decision

Two crates, not one — following the `edge-runtime-http` / `edge-llm-runtime` split already established by ADR-045 and the top-level `edge-runtime-*` ADR-047, because MCP genuinely has two different concerns at two different coupling levels:

1. **`edge-runtime-mcp`** (`runtime/mcp/`, in the `sweengineeringlabs/edge-runtime` repo, alongside `runtime/http/`, `runtime/grpc/`) — the **wire-protocol contract**: JSON-RPC 2.0 message shapes, tool listing, tool invocation, all as opaque `serde_json::Value` payloads. Zero knowledge of `Skill`, `Handler`, or anything LLM-domain-specific. This is transport-agnostic-of-domain in exactly the sense `edge-runtime-http`'s `HttpIngress` is: an HTTP server has no idea it's carrying LLM traffic, and an MCP tool-call gateway has no idea whether the tool underneath is a `Skill`, a shell script, or a database query. Concrete stdio/SSE JSON-RPC framing (the "transport crate," analogous to `swe-edge-ingress-http`'s Axum binding) is explicitly out of scope for this crate and this ADR — see What This ADR Explicitly Does NOT Solve.
2. **`edge-llm-mcp`** (`domain/scm/domain/llm/mcp/`) — the **LLM-domain adapter**: translates between `edge-runtime-mcp`'s wire types and `edge-llm-agent`'s `Skill`/`Handler` types. This is the crate that is actually coupled to agent/skill semantics, and it belongs in `domain/scm/domain/llm/` for the same reason `edge-llm-runtime` (ADR-045) is a separate concern from `swe-edge-bootstrap`: the wire protocol doesn't know about skills, but the translation from "a `Skill`'s `SkillMetadata`" to "an MCP `Tool` descriptor" is 100% LLM-domain knowledge with no home in a protocol-only crate.

This mirrors the `HttpIngress`(protocol) / `LlmSkillHandler`(domain adapter, ADR-046 lineage) split precisely, and answers the "where does this belong" question the same way ADR-047 (top-level) answered it for HTTP/gRPC: **the transport is domain-agnostic; the adapter is not.** MCP is *more* coupled to agent/skill semantics than HTTP/gRPC (its whole payload vocabulary — "tools," "resources," "prompts" — is LLM-native), which is exactly why a thin two-crate split is used instead of jamming everything into one side: put the genuinely protocol-only 20% in `edge-runtime-mcp` (so a non-LLM consumer, e.g. exposing `ProcessRunner` from ADR-003 directly as an MCP tool with no `Skill` involved at all, is still possible), and put the domain-coupled 80% in `edge-llm-mcp` where `Skill`/`SkillMetadata` already live.

### Shape / workspace layout

#### `edge-runtime-mcp` (contracts only, no `core/`, no transport dependency — same tier discipline as `edge-runtime-http`)

```
runtime/mcp/scm/
└── main/src/
    ├── api/
    │   ├── traits/
    │   │   ├── mcp_tool_ingress.rs   # server-side: expose tools
    │   │   └── mcp_tool_egress.rs    # client-side: call tools on a remote server
    │   ├── types/
    │   │   ├── mcp_tool_definition.rs
    │   │   ├── mcp_list_tools_{request,response}.rs
    │   │   ├── mcp_call_tool_{request,response}.rs
    │   │   ├── mcp_list_remote_tools_{request,response}.rs
    │   │   └── mcp_call_remote_tool_{request,response}.rs
    │   └── errors/mcp_tool_error.rs
    └── saf/
        ├── mcp_tool_ingress_svc.rs
        └── mcp_tool_egress_svc.rs
```

```rust
/// Server-side: exposes a set of tools to an external MCP client
/// (Claude Code, Claude Desktop, another agent process).
pub trait McpToolIngress: Send + Sync {
    fn list_tools(&self, req: McpListToolsRequest) -> Result<McpListToolsResponse, McpToolError>;
    fn call_tool(&self, req: McpCallToolRequest) -> Result<McpCallToolResponse, McpToolError>;
}

/// Client-side: calls tools hosted on an arbitrary external MCP server.
pub trait McpToolEgress: Send + Sync {
    fn list_remote_tools(
        &self,
        req: McpListRemoteToolsRequest,
    ) -> Result<McpListRemoteToolsResponse, McpToolError>;

    fn call_remote_tool(
        &self,
        req: McpCallRemoteToolRequest,
    ) -> Result<McpCallRemoteToolResponse, McpToolError>;
}
```

```rust
pub struct McpToolDefinition {
    pub name: String,
    pub description: String,
    /// JSON Schema, serialized — same representation `SkillMetadata.input_schema` already uses.
    pub input_schema: String,
}

pub struct McpCallToolRequest {
    pub tool_name: String,
    /// Opaque JSON payload — this crate does not know or care what's inside.
    pub arguments: serde_json::Value,
}

pub struct McpCallToolResponse {
    pub content: serde_json::Value,
    pub is_error: bool,
}

pub enum McpToolError {
    UnknownTool(String),
    InvalidArguments(String),
    Transport(String),
    Denied(String),
}
```

`McpToolError` ends in `Error` (satisfies `api_error_type_named`). No `SecurityContext`/`ObserverContext`/`HandlerContext` reference in this crate at all — those belong on the domain side, exactly like `HttpIngress`/`GrpcIngress` carry no LLM-specific context either.

#### `edge-llm-mcp` (the domain adapter — depends on both `edge-runtime-mcp` and `edge-llm-agent`)

```
domain/scm/domain/llm/mcp/
├── api/
│   ├── traits/
│   │   └── mcp_skill_bridge.rs        # SkillMetadata <-> McpToolDefinition conversion contract
│   ├── types/
│   │   ├── skill_to_tool_{request,response}.rs
│   │   └── remote_tool_skill_{request,response}.rs
│   └── errors/mcp_bridge_error.rs
├── core/
│   ├── handler_registry_mcp_ingress.rs  # McpToolIngress impl backed by a HandlerRegistry of Skills
│   └── remote_mcp_skill.rs              # Skill/Handler impl backed by an McpToolEgress + tool name
└── saf/
```

```rust
/// Server direction: adapts a registry of registered Skills into an McpToolIngress.
/// `execute()` on a matched tool call delegates to `Handler::execute` — any Skill
/// wrapped in ADR-046's `GovernedHandler<H>` before registration is gated automatically;
/// this adapter does not need its own governance logic.
pub(crate) struct HandlerRegistryMcpIngress {
    registry: Arc<dyn HandlerRegistry>,
}

impl McpToolIngress for HandlerRegistryMcpIngress {
    fn list_tools(&self, req: McpListToolsRequest) -> Result<McpListToolsResponse, McpToolError> { ... }
    fn call_tool(&self, req: McpCallToolRequest) -> Result<McpCallToolResponse, McpToolError> { ... }
}

/// Client direction: wraps a remote MCP tool as an ordinary Skill/Handler, so any
/// Agent invokes it exactly like a locally-defined skill — generalizing what
/// llmboot's tool_invoke does bespoke against only the ADS command server today.
pub(crate) struct RemoteMcpSkill {
    egress: Arc<dyn McpToolEgress>,
    tool_name: String,
}

impl Skill for RemoteMcpSkill { ... }
#[async_trait]
impl Handler for RemoteMcpSkill {
    type Request = serde_json::Value;
    type Response = serde_json::Value;

    async fn execute(
        &self,
        req: ExecutionRequest<'_, Self::Request>,
    ) -> Result<Self::Response, HandlerError> {
        self.egress
            .call_remote_tool(McpCallRemoteToolRequest {
                tool_name: self.tool_name.clone(),
                arguments: req.req,
            })
            .map(|resp| resp.content)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))
    }
}
```

Depends on: `edge-domain-handler` (`Handler`, `HandlerContext`, `HandlerRegistry`, `HandlerError`, `ExecutionRequest`), `edge-runtime-mcp` (`McpToolIngress`, `McpToolEgress`, wire types), `edge-llm-agent` (`Skill`, `SkillMetadata`, `ParameterDocumentation`), `edge-security-runtime` (`SecurityContext` — threaded transparently via `HandlerContext`, no new context type), `edge-domain-observer` (`ObserverContext` — same span/counter seam ADR-044/ADR-046 already use).

### Server vs. client scope for v1

Both directions are designed in full above; only the **server** direction (`HandlerRegistryMcpIngress`) ships in v1:

- **Server** closes the actual audit finding — "no edge-native MCP port exists" — by letting any registered `Skill`/`Handler` (already running inside `edge-llm-runtime`, ADR-045) be discovered and called by an external MCP client (Claude Code, Claude Desktop, another agent). Nothing in `edge` can do this today. It is also the smaller lift: `McpToolIngress` only needs to enumerate already-existing `SkillMetadata` and dispatch already-existing `Handler::execute` calls — no new network client, no remote-server bookkeeping.
- **Client** (`RemoteMcpSkill`) generalizes llmboot's existing bespoke ADS-only MCP client to an arbitrary external MCP server — genuinely useful (an `edge-llm-agent` `Agent` could call a third-party MCP tool server without edge growing a bespoke per-server adapter each time), but it duplicates capability llmboot already has for the one server that matters today (ADS), and it requires the actual stdio/SSE JSON-RPC transport implementation (the "transport crate" analogue of `swe-edge-ingress-http`) to exist first — that transport crate is out of scope here (see below) and blocks `McpToolEgress` from having a real, non-mock implementation. Building the client-side contract now (so it's designed once, correctly) while deferring its concrete transport is the same phasing discipline ADR-045 used for prompt/reasoning/agent `saf/` factories: mechanical follow-up, not a design question, once the transport crate lands.

## What this ADR explicitly does NOT solve

- **The actual JSON-RPC/stdio/SSE wire transport.** Neither `edge-runtime-mcp` nor `edge-llm-mcp` implements the byte-level MCP protocol (message framing, capability negotiation handshake, notifications). That is a `swe-edge-ingress-mcp`-shaped transport crate (analogous to `swe-edge-ingress-http` wrapping `edge-runtime-http`'s `HttpIngress`), explicitly deferred — this ADR defines the contract it would bind to, not the binding itself.
- **`McpToolEgress`'s real implementation.** v1 ships `HandlerRegistryMcpIngress` (server) only; `RemoteMcpSkill` (client) is designed but has no shipped, non-mock `McpToolEgress` behind it until the transport crate above exists.
- **Replacing ADS's `swe_agentdevstudio_mcp_server_command` or llmboot's `tool_invoke` executor.** Both keep working exactly as-is. This ADR gives `edge` its own MCP surface; it does not migrate or deprecate the ADS/llmboot pair ADR-003 already covers. ADR-003's process-runner conclusion — "the MCP server remains the right path for LLM-dispatched tool calls" — is unaffected; this ADR is about `edge` gaining that same capability natively, not replacing ADS's.
- **Per-tool capability/risk declarations for MCP-exposed skills.** `ToolCapabilityModel` (ADR-046) is the mechanism; whether every skill exposed via `HandlerRegistryMcpIngress` must declare one before being MCP-exposed is a registration-time policy decision left to the composition root (`edge-llm-runtime`, ADR-045), same as ADR-046 already leaves "does this handler get wrapped in `GovernedHandler`" unresolved as an opt-in choice, not an automatic default.
- **MCP "resources" and "prompts" primitives** (MCP's non-tool-call message types). Only `tools/list` and `tools/call` are in scope — the two that map onto `Skill`/`Handler`. Resources/prompts have no existing edge-domain analog and are out of scope until a concrete consumer needs them.
- **Multi-tenant MCP server discovery/registry** (a directory of known external MCP servers an agent could browse). `McpToolEgress` addresses one already-known server at a time, same granularity as ADS today.

## Consequences

**What this enables**
- `edge` gains a native way to expose any registered `Skill`/`Handler` to an external MCP client (Claude Code, Claude Desktop, a third-party agent) — a capability that exists nowhere in `edge` today (confirmed: zero MCP references anywhere in the monorepo outside this ADR and the two ADR-003 citations to ADS/llmboot, which are both external to `edge`).
- Every MCP-exposed tool automatically inherits `SecurityContext`/`ObserverContext` and — if the composition root chooses to wrap it — `GovernedHandler`'s capability/risk gate, with zero MCP-specific governance code: this is a direct reuse validation of ADR-046, not new machinery.
- `SkillMetadata`/`ParameterDocumentation` (already-shipped types in `edge-llm-agent`) become directly reusable as the MCP tool descriptor source of truth — no parallel "MCP tool schema" vocabulary to keep in sync.
- A named, designed target for llmboot's eventual generalization from "bespoke ADS-only MCP client" to "any MCP server," without committing to build it before its transport dependency exists.
- `edge-runtime-mcp` stays reusable by non-LLM consumers too — e.g., `ProcessRunner` (ADR-003) could be exposed as an MCP tool directly, without ever touching `Skill`, exactly the way ADS's own `CommandServer` already does it today outside `edge`.

**What this requires**
- New crate `edge-runtime-mcp` under `runtime/mcp/` in the `sweengineeringlabs/edge-runtime` repo — a new workspace member, following the exact contracts-only tier discipline the top-level `edge-runtime-*` ADR-047 already prescribes (and the same tier `edge-runtime-http`/`edge-runtime-grpc`/`edge-runtime-cli` already occupy in that repo today).
- New crate `edge-llm-mcp` under `domain/scm/domain/llm/mcp/`, depending on `edge-domain-handler`, `edge-runtime-mcp`, `edge-llm-agent`, `edge-security-runtime`, `edge-domain-observer` — no changes to any of their existing traits.
- A follow-on, explicitly separate transport crate (name TBD, e.g. `swe-edge-ingress-mcp`) to actually speak JSON-RPC 2.0 over stdio/SSE — not scoped here, tracked as a prerequisite for the client direction and for making the server direction reachable by a real external process rather than only unit-testable in-process.
- `edge-llm-runtime` (ADR-045) gains one more `saf/`-style registration point once `edge-llm-mcp` exists: wiring `HandlerRegistryMcpIngress` alongside the existing HTTP/gRPC routes — mechanical, not a design change, same as ADR-045's per-crate `saf/<theme>_handler_svc.rs` follow-ups.
- No changes to `edge-domain-handler`, `edge-domain-observer`, `edge-security-runtime`, `edge-llm-agent`'s existing traits, or ADR-046's `GovernedHandler`.

## Alternatives Considered

**Single `edge-llm-mcp` crate, no `edge-runtime-mcp` split**
Rejected. Would tie the wire protocol (tool listing/calling as opaque JSON) to `edge-llm-agent`'s `Skill`/`Handler` types, making it impossible to expose anything that isn't a `Skill` (e.g. `ProcessRunner` from ADR-003) as an MCP tool without a dependency on the LLM domain. The top-level `edge-runtime-*` ADR-047 rejected exactly this shape of coupling for HTTP/gRPC ("plugins are not allowed to depend on transport infrastructure... this option is a dependency inversion violation") — the same reasoning applies here in reverse: a protocol-only consumer shouldn't have to depend on the LLM domain to speak MCP.

**Put the whole port in `domain/scm/domain/llm/` as one `edge-llm-mcp` crate that also owns the wire types**
Considered, since MCP's vocabulary ("tools," "resources") is more LLM-native than HTTP/gRPC's, arguably blurring the transport/domain line more than those two. Rejected anyway: even granting MCP is *more* domain-coupled than HTTP, the tool-listing/tool-calling JSON-RPC shape itself (request id, method name, params, result, error) carries zero `Skill`-specific meaning — it is still possible (and per ADS's own `CommandServer` precedent, already done in practice) to expose a non-`Skill` capability as an MCP tool. Collapsing the two would foreclose that reuse for no simplification benefit — the split costs one extra crate and one extra `Cargo.toml`, not real design complexity.

**Build only the client direction (generalize llmboot), skip the server direction**
Rejected for v1 ordering, not rejected outright — see Decision's "Server vs. client scope for v1." The server direction is the one gap with literally zero existing capability anywhere (llmboot's client already exists, bespoke, against ADS); building the direction with an existing analog first and deferring the one blocked on an unbuilt transport crate is the same "prove plumbing, defer the harder half" sequencing ADR-045 used for its own vendor-`Completer` gap.

**Build both directions with a shared, single trait (`McpTool: ingress + egress` combined)**
Rejected. A server exposing tools and a client calling a remote server's tools have disjoint call shapes (`list_tools`/`call_tool` take no remote-endpoint parameter; `list_remote_tools`/`call_remote_tool` need one) and disjoint failure modes (`Denied` is a server-side governance outcome; `Transport` failures are overwhelmingly client-side). Splitting into `McpToolIngress`/`McpToolEgress` mirrors `HttpIngress`/`HttpEgress`'s existing ingress/egress split in the top-level ADR-047's own table, rather than inventing a new merged shape.

## Tracking

- New crate: `edge-runtime-mcp` (`runtime/mcp/` in `sweengineeringlabs/edge-runtime`) — `McpToolIngress`, `McpToolEgress`, wire types, `McpToolError`
- New crate: `edge-llm-mcp` (`domain/scm/domain/llm/mcp/`) — `HandlerRegistryMcpIngress` (server, v1), `RemoteMcpSkill` (client, designed, not shipped until transport crate lands)
- Follow-up, separate ADR/issue: MCP stdio/SSE transport crate (e.g. `swe-edge-ingress-mcp`), analogous to `swe-edge-ingress-http` — prerequisite for `McpToolEgress`'s real implementation and for the server direction being reachable outside the same process
- Follow-up: register `HandlerRegistryMcpIngress` in `edge-llm-runtime` (ADR-045) once both this ADR and ADR-045 are accepted
- Follow-up: audit whether `edge-runtime-mcp` should be proposed back into the top-level `edge-runtime-*` family table (`docs/3-architecture/adr/ADR-047-edge-runtime-primitive-family.md`) as an eleventh phase, since that ADR's author is unaware MCP is missing from its own list
- Not tracked here, unaffected: ADS's `swe_agentdevstudio_mcp_server_command`, llmboot's `tool_invoke` executor (ADR-003's existing scope)
