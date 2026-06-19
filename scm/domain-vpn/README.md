# edge-domain-vpn

VPN port contracts for the `swe-edge` domain layer.

Provides:
- `TunnelManager` — async trait for connect/disconnect/status lifecycle
- `VpnClientConfig` — `[vpn]` `OptionalSection` TOML toggle (ADR-041)
- `NoopTunnelManager` — no-op for tests and non-unix platforms
- `VpnManagerFactory` — SAF factory methods

## Usage

Add `[vpn]` to `application.toml` to activate:

```toml
[vpn]
endpoint          = "vpn.internal:51820"
server_public_key = "BASE64_ENCODED_32_BYTE_X25519_KEY"
```

See `hello-vpn` example for a full end-to-end demo.
