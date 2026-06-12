# edge-domain

L2 Domain contract for the SWE Edge platform.

Defines `Handler`, `HandlerRegistry`, `RequestContext`, `HandlerError`, and supporting event-sourcing, repository, service, and command/query port contracts. All public surface is delegated via the SAF facade (`saf/`).

## Usage

```toml
[dependencies]
edge-domain = { git = "https://github.com/sweengineeringlabs/edge-domain" }
```

## Architecture

See `docs/` for ADRs and sequence diagrams. Port contracts are owned here; concrete implementations live in adapter crates that depend on this crate.
