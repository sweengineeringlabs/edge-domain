# ADR-068: `CacheableMessage` Consumer — Wiring `cache_control` into `AnthropicCompleter`'s Request Mapping

**Status:** Proposed
**Date:** 2026-07-08
**Author:** Senior Agentic Engine Engineer
**Relates to:** ADR-043 (LLM Complete), ADR-048 (Real Vendor Completer — the actual consumer this ADR wires into), ADR-066 (`with_cached_system()` builder — complementary producer-side convenience, not overlapping)
**GitHub Issues:** TBD

---

## Context

A 2026-07-08 audit of the LLM-complete landscape found a "declare and abandon" field (CLAUDE.md anti-pattern #10): `Message.cache_control: Option<CacheControl>` (`domain/scm/domain/llm/complete/main/src/api/complete/types/message.rs:19`) is a real, typed, documented field — but nothing anywhere reads it.

### The field, verified in full

- **`CacheControl`** (`domain/scm/domain/llm/complete/main/src/api/complete/types/cache_control.rs:4-9`): a single-field struct, `cache_type: String`, serialized as JSON `"type"` (`#[serde(rename = "type")]`, line 7-8). Its only constructor today is `CacheControl::ephemeral()` (`core/complete/cache_control.rs:14-16`), which normalizes to the literal string `"ephemeral"`. That is exactly Anthropic's real Messages API cache-breakpoint shape — a content block carries `"cache_control": {"type": "ephemeral"}` to mark "cache everything up to and including this block." The domain type does **not** model Anthropic's newer optional `ttl` sub-field (`"5m"`/`"1h"`); `cache_type` is a free-form `String`, not a closed enum, so today only `"ephemeral"` is ever produced, but the shape doesn't prevent other values being set by hand — see "What this ADR explicitly does NOT solve."
- **`CacheableMessage`** trait (`api/complete/traits/cacheable_message.rs:9-25`): `with_cache_control(self, CacheControlRequest) -> Result<CacheControlResponse<Self>, CompleteError>` plus a default `mark_ephemeral(self, MarkEphemeralRequest)` convenience that calls it with `CacheControl::ephemeral()` (lines 17-24). Already Request/Response-shaped, already `*Error`-suffixed — nothing to redesign here.
- **`impl CacheableMessage for Message`** (`core/complete/message.rs:47-54`): sets `self.cache_control = Some(*req.cache)` and returns it wrapped. Unit-tested in isolation (`message.rs:86-108`; also `complete/tests/cacheable_message_e2e_test.rs`, `cacheable_message_svc_e2e_test.rs`, `cache_control_e2e_test.rs`) — every one of these tests only asserts the field got set on the Rust struct. None of them touches JSON, HTTP, or a vendor payload.
- **Where it lives in the request**: `CompletionRequest.messages: Vec<Message>` (`api/complete/types/completion_request.rs:11`). `Message.content: MessageContent` (`message.rs:11`) is `Empty | Text(String) | Parts(Vec<ContentPart>)` (`message_content.rs:8-16`, `#[serde(untagged)]`), and `ContentPart` (`content_part.rs:15-33`) is `Text{text} | ImageUrl{..} | ImageBase64{..}` — **`cache_control` lives on `Message`, not per-`ContentPart`.** This matters for the mapping design below, because Anthropic's real API attaches `cache_control` to individual content *blocks*, not to a top-level message object.

### Verifying the audit's "zero consumers" claim

`grep -r cache_control domain/scm/domain/llm` returns 71 hits. Every one resolves to exactly one of:
1. The `complete` crate's own type/trait/test files listed above.
2. A **sibling, independently-defined, equally-unconsumed mirror** in the `agents` crate: `agents/main/src/api/types/cache_control.rs:4-9` (byte-identical shape), `agents/main/src/api/types/message.rs:20-22` (same `cache_control: Option<CacheControl>` field, with `#[serde(skip_serializing_if = "Option::is_none")]` — meaning even if this were serialized, empty means omitted, correctly, but again: never actually put on a wire anywhere), and `agents/main/src/core/types/message_builder.rs:55-56` (a builder setter, same story).
3. Structural-audit JSON report artifacts (`agents/docs/7-operations/compliance/structural_audit_report_*.json`) — audit-tool noise, not code.

`domain/scm/domain/llm/complete/main/src/spi/mod.rs:1-9` confirms there is **no request-mapping code in this repo at all** — it is a one-line extension anchor (`const _: () = ();`) with a doc comment stating "downstream crates implement the `Completer` contract... reference implementations live in `core/`." A grep of `provider/` and of `complete/main/src/spi/` for `cache_control` returns nothing. There is no HTTP/JSON layer anywhere in `domain/` for this field to reach — by design, per ADR-042's plugin-boundary rule, that layer doesn't exist until `edge-plugin-llm-anthropic` (ADR-048) is built. This ADR confirms the audit's claim exactly: the field is real, correctly typed, unit-tested — and, once you leave the `complete`/`agents` crates, invisible.

### Does ADR-048 already account for it?

ADR-048 (Proposed, same date) designs `AnthropicCompleter`'s `spi/anthropic_request.rs` (`ADR-048-real-vendor-completer.md:113`) as the file that maps `CompletionRequest` → Anthropic Messages JSON. Its `complete` mapping bullet (`ADR-048-real-vendor-completer.md:124`) specifies role mapping (`User`→`"user"`, `Assistant`→`"assistant"`, system hoisted to top-level `system` field) and error-path mapping in detail — but **says nothing about `cache_control`**. ADR-048's "What this ADR explicitly does NOT solve" section (line 153) does mention prompt caching, but narrowly and on the *response* side only: *"prompt caching (`cache_read_input_tokens`/`cache_creation_input_tokens` wiring) — `TokenUsage`'s existing fields for these are populated from Anthropic's response where present, but no consumer aggregates or bills against them."* That is about billing telemetry coming back from Anthropic, not about `Message.cache_control` going out. **ADR-048 silently drops the request-side field — it is not named as an explicit gap, it is simply absent from the mapping design.** That silent omission is exactly what this ADR closes.

## Decision

Amend ADR-048's `spi/anthropic_request.rs` design (not yet built — no code exists to change today, only the design) to add one mapping step: **the message-body converter reads each `Message.cache_control` via the already-existing `CacheableMessage`-populated field and, when `Some`, emits a `cache_control` object on the last Anthropic content block belonging to that message.**

### Where exactly this plugs in

`spi/anthropic_request.rs`'s per-message conversion (the function ADR-048 describes informally as "role/content mapping," `ADR-048-real-vendor-completer.md:124`) already has to turn `Message.content: MessageContent` into Anthropic's content-block JSON array, because Anthropic's Messages API always wants `content` as an array of typed blocks (`{"type": "text", "text": "..."}`, etc.), even for a single string. That conversion is the one and only place a `cache_control` block can attach, because Anthropic's real API puts `cache_control` *inside* a content block, not on the enclosing message object:

```json
{
  "role": "user",
  "content": [
    { "type": "text", "text": "…", "cache_control": { "type": "ephemeral" } }
  ]
}
```

Concretely, in `spi/anthropic_request.rs`:

1. `fn message_to_anthropic_blocks(msg: &Message) -> Vec<serde_json::Value>` — converts `Message.content` (`MessageContent::Empty|Text|Parts`) into the block array unconditionally, exactly as ADR-048 already requires for any message to be sent at all.
2. **New step**: if `msg.cache_control.is_some()`, attach that `CacheControl` (serialized via its existing `#[serde(rename = "type")]`, `cache_control.rs:7-8` — no new serialization code needed, `serde_json::to_value(&cc)` already produces `{"type": "ephemeral"}`) onto the **last** block in the array returned by step 1. Anthropic's cache-breakpoint semantics are "cache everything up to and including this marked block," so attaching to the last block of the message is the correct, minimal mapping for a message-level (not sub-block-level) cache hint — consistent with `Message` only ever carrying one `cache_control` value (`message.rs:19`), not one per `ContentPart`.
3. For `Role::System` messages hoisted to the top-level `system` field (per ADR-048's existing rule), the same rule applies: `system` becomes an array of blocks (Anthropic accepts this form), and a system message's `cache_control` lands on the last system block the same way — no special-casing needed beyond routing through the same `message_to_anthropic_blocks` helper before the hoist.
4. If `msg.cache_control` is `None` (the common case — most messages aren't cache breakpoints), no `cache_control` key is emitted at all — matching Anthropic's API, which treats the key's absence as "no caching hint," not "disable caching."

No change to `edge-llm-complete`'s `api/` is needed: `CacheControl`, `CacheableMessage`, `CacheControlRequest`/`Response`, `MarkEphemeralRequest` are all already correctly shaped and already sufficient (same "zero `api/` changes" posture ADR-048 itself takes for its own scope, `ADR-048-real-vendor-completer.md:168`).

### Provable before any live vendor call

Per ADR-045/048's "prove the plumbing before the real backend" posture: this mapping is a pure function (`Message` in, `serde_json::Value` out) with no network dependency, so it is fully unit-testable the moment `spi/anthropic_request.rs` exists, independent of whether `AnthropicCompleter::complete` has ever made a real HTTP call. The concrete test to add once ADR-048's plugin repo is scaffolded:

```rust
// edge-plugin-llm-anthropic/scm/tests/anthropic_request_cache_control_test.rs
#[test]
fn test_message_to_anthropic_blocks_with_cache_control_emits_cache_control_block() {
    let msg = Message::user("long context…")
        .mark_ephemeral(MarkEphemeralRequest)
        .expect("mark_ephemeral ok")
        .message;
    let blocks = message_to_anthropic_blocks(&msg);
    let last = blocks.last().expect("at least one block");
    assert_eq!(last["cache_control"]["type"], "ephemeral");
}

#[test]
fn test_message_to_anthropic_blocks_without_cache_control_omits_key() {
    let msg = Message::user("no caching here");
    let blocks = message_to_anthropic_blocks(&msg);
    assert!(blocks.last().expect("at least one block").get("cache_control").is_none());
}
```

This asserts the exact JSON shape Anthropic's API requires, using only the types that already exist in `edge-llm-complete` today (`Message::user`, `mark_ephemeral`, `MarkEphemeralRequest`) — nothing here waits on ADR-048's HTTP/SSE machinery to be functional.

## What this ADR explicitly does NOT solve

- **The `AnthropicCompleter`/`edge-plugin-llm-anthropic` repo itself** — that is ADR-048's scope, still not built. This ADR only amends ADR-048's `spi/anthropic_request.rs` design; there is no code to merge until that repo exists.
- **The `agents` crate's mirrored `cache_control` field** (`agents/main/src/api/types/message.rs:20-22`, `agents/main/src/core/types/message_builder.rs:55-56`) — same "declare and abandon" shape, but the `agents` crate's `Message` is not what `Completer::complete` consumes (`CompletionRequest.messages: Vec<edge_llm_complete::Message>`, `completion_request.rs:11`); whatever bridges an agent conversation into a `CompletionRequest` (if anything currently does) is a separate, uninvestigated gap, flagged for tracking, not fixed here.
- **`CacheControl`'s `ttl` sub-field** — Anthropic's API also accepts an optional `"ttl": "5m"|"1h"` alongside `"type": "ephemeral"`; `CacheControl` (`cache_control.rs:4-9`) has no such field today. Since `cache_type` is a plain `String`, adding a `ttl: Option<String>` field later is additive and non-breaking, but it is not part of this ADR — v1 assumes the default (5-minute) TTL Anthropic applies when `ttl` is omitted.
- **The 4-cache-breakpoint limit** — Anthropic's API rejects requests with more than four `cache_control`-marked blocks. This ADR's mapping is a correct pass-through (one breakpoint per `Message` that has `cache_control: Some(..)`), but adds no validation that a caller hasn't marked more than four messages. A caller who over-marks gets Anthropic's own 400 error back through the existing `CompleteError::ProviderError` path (ADR-048's error mapping, `ADR-048-real-vendor-completer.md:124`) — no new error variant is added for this, matching ADR-048's stance that the vendor's own error response is an acceptable backstop for API-shape constraints not worth pre-validating client-side.
- **Response-side cache telemetry** — `TokenUsage.cache_read_input_tokens`/`cache_creation_input_tokens` wiring remains exactly the gap ADR-048 already named (`ADR-048-real-vendor-completer.md:153`) and is unaffected by this ADR; this ADR is request-side only.
- **`with_cached_system()` producer-side convenience (ADR-066)** — that ADR (if/when merged) adds a call-site builder that *sets* `cache_control` more ergonomically; it does not change how the field is consumed. The two ADRs are complementary: ADR-066 makes the field easier to set, this ADR makes a real consumer read what was set. Neither depends on the other shipping first — `mark_ephemeral`/`with_cache_control` (already implemented) are sufficient inputs to this ADR's mapping regardless of whether ADR-066 ships.
- **Any change to `edge-llm-complete`'s `api/`** — as noted in Decision, zero changes needed; `CacheControl`/`CacheableMessage` are already correctly shaped.

## Consequences

**What this enables**
- Closes the specific "declare and abandon" finding from the 2026-07-08 audit: once ADR-048's plugin exists, `cache_control` set via `mark_ephemeral`/`with_cache_control` will actually reduce token costs and latency against real Anthropic traffic, instead of being silently discarded at the request-mapping boundary.
- A concrete, minimal, additive amendment to ADR-048's `spi/anthropic_request.rs` design — one helper function gains one conditional branch; no new trait, no new port, no new error variant.
- The mapping is provable in a unit test today's `edge-llm-complete` types already support (see "Provable before any live vendor call"), so correctness of the JSON shape doesn't have to wait on live Anthropic credentials or ADR-048's HTTP/SSE plumbing being finished.

**What this requires**
- When `edge-plugin-llm-anthropic` (ADR-048) is scaffolded, its `spi/anthropic_request.rs` message-to-block converter must include the `cache_control`-on-last-block step described here — a reviewer checklist item against ADR-048's PR, not separate new work streams.
- No changes to `edge-llm-complete`, `edge-llm-agent`, or any other already-shipped crate.

## Alternatives Considered

**Attach `cache_control` to every block of a multi-part message, not just the last one**
Rejected. Anthropic's own guidance is that a `cache_control` marker denotes a breakpoint — "cache up to and including this block" — not "cache this block specifically." Marking every block would be redundant (and, per Anthropic's 4-breakpoint cap, would burn through the budget on a single message for no benefit) versus marking only the last block, which achieves the same caching outcome with one breakpoint instead of N.

**Add a `cache_control: Option<Vec<CacheControl>>` per-`ContentPart` field instead, for full block-level granularity**
Rejected for now. This would let a caller mark an arbitrary interior block as a breakpoint (not just "the end of this message"), which is a real Anthropic capability this design doesn't expose. But it requires an `api/` change to `ContentPart` (`content_part.rs`) and a new `CacheableMessage`-style trait for it, which no known caller in this monorepo currently needs — `Message`-level granularity already matches every existing test and usage. Deferred as a follow-on if a concrete multi-breakpoint-per-message use case shows up; not invented speculatively here (CLAUDE.md's "declare and abandon" anti-pattern applies just as much to over-engineering the fix as to under-wiring the original field).

**Fix this by changing `edge-llm-complete`'s `api/` (e.g., moving `cache_control` onto `ContentPart`)**
Rejected. Per the audit, the field's *shape* is not the defect — it is correctly typed, matches Anthropic's `{"type": "ephemeral"}` block shape, and is already unit-tested. The defect is purely "no consumer reads it," which is a `spi/` mapping gap in a not-yet-built plugin, not an `api/` design gap. Changing `api/` here would be scope creep against a part of the system that already works.

**Leave `cache_control` unconsumed until ADR-048's plugin ships, then decide the mapping "when we get there"**
Rejected. This is exactly the posture ADR-045/048 argue against ("prove the plumbing before the real backend"): specifying the mapping now, while it's cheap to review and test against pure JSON assertions, is strictly better than re-discovering the same silent-drop gap during ADR-048's own implementation and papering over it under time pressure.

## Tracking

- Depends on: ADR-048 (`edge-plugin-llm-anthropic` repo, not yet created) — this ADR's fix has no code home until that repo's `spi/anthropic_request.rs` exists; track as a required checklist item on that ADR's implementation, not a separate blocking issue.
- Complementary, not blocking either direction: ADR-066 (`with_cached_system()` builder)
- Follow-up (separate, uninvestigated): whether/how the `agents` crate's mirrored `cache_control` field (`agents/main/src/api/types/message.rs:20-22`) ever reaches a `CompletionRequest` at all — flagged, not fixed here
- Follow-up (optional, non-blocking): `CacheControl.ttl` sub-field, if a caller ever needs non-default TTL
- Not blocking this ADR: 4-breakpoint-limit client-side validation (vendor's 400 response is an acceptable backstop, per Alternatives)
