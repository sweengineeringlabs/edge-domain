# Changelog — edge-domain-vpn

## [0.1.0] — 2026-06-19

### Added
- `TunnelManager` async trait: `connect`, `disconnect`, `status`
- `VpnClientConfig` `OptionalSection` for `[vpn]` TOML toggle (ADR-041)
- `NoopTunnelManager` — no-op implementation for tests and non-unix platforms
- `VpnManagerFactory::noop_tunnel_manager()` SAF factory
- `TunnelStatus` enum: Disconnected, Connecting, Handshaking, Connected, Reconnecting, Failed
- `VpnError` with ConfigError, ConnectionFailed, TunnelError, Unavailable variants
