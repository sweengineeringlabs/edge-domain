# edge-domain architecture

SEA-layout single-crate library.

- `api/` — public declarations: `Handler`, `HandlerRegistry`, `OutboundRegistry`, `HandlerError`
- `core/` — implementations of all api/ types
- `saf/` — public facade, factory functions
- `gateway/` — domain entry-point adapters (input/output)
