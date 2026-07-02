//! Implementation home for `api/traits/` — currently a stub.
//!
//! All 7 traits declared in `api/traits/` (`Agent`, `AgentLifecycle`, `AgentManager`,
//! `AgentRegistry`, `SchemaValidator`, `Skill`, `Validator`) are implemented directly
//! against their owning types in `core/noop/` (for the no-op reference implementations)
//! and `core/types/` (for `DefaultAgentHandler`). Production implementations live in
//! plugins (e.g. `edge-plugin-llmboot`), not here.
