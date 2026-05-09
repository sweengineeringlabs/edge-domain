# edge-domain

L2 Domain contract for the `swe-edge` framework.

Defines the `Handler` trait and `HandlerRegistry` — the execution-unit building blocks
consumed by the gateway, ingress, and egress layers. No knowledge of transport protocols.

## Usage

```rust
use edge_domain::{Handler, HandlerRegistry, new_handler_registry};
```
