//! `VpnClientConfig` — `OptionalSection` TOML contract for `[vpn]`.

use std::net::SocketAddr;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use swe_edge_configbuilder::{ConfigError, FeatureMetadata, OnError, OptionalSection};

/// TOML section contract for the `[vpn]` opt-in.
///
/// Presence of `[vpn]` in `application.toml` activates the tunnel.
/// Absence leaves the runtime running without any VPN tunnel (zero overhead).
///
/// # Security
///
/// `server_public_key` and `psk` are validated (base64-decoded, length-checked)
/// during `validate_enabled()`. A malformed key aborts startup rather than
/// allowing a silent WireGuard handshake failure — `on_error() = OnError::Fail`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VpnClientConfig {
    /// WireGuard endpoint of the swelvpn server (`host:port`).
    pub endpoint: SocketAddr,

    /// Server's static X25519 public key — base64-encoded, 32 bytes decoded.
    ///
    /// Required: WireGuard handshake fails silently without a peer public key.
    pub server_public_key: String,

    /// Optional pre-shared key — base64-encoded, 32 bytes decoded.
    ///
    /// Omit for no PSK. Maps to `VpnConfig::psk` at backend conversion time.
    #[serde(default)]
    pub psk: Option<String>,

    /// Seconds between keepalive packets.
    #[serde(default = "VpnClientConfig::default_keepalive")]
    pub keepalive_interval: u64,

    /// Seconds before a handshake attempt is abandoned.
    #[serde(default = "VpnClientConfig::default_handshake_timeout")]
    pub handshake_timeout: u64,

    /// Reconnect automatically on connection loss.
    #[serde(default = "VpnClientConfig::default_auto_reconnect")]
    pub auto_reconnect: bool,

    /// Maximum reconnect attempts; `0` = unlimited.
    #[serde(default)]
    pub max_reconnect_attempts: u32,

    /// Log VPN events verbosely.
    #[serde(default)]
    pub verbose: bool,
}

impl VpnClientConfig {
    /// Default value for `keepalive_interval` (25 seconds).
    pub fn default_keepalive() -> u64 {
        25
    }

    /// Default value for `handshake_timeout` (10 seconds).
    pub fn default_handshake_timeout() -> u64 {
        10
    }

    /// Default value for `auto_reconnect` (true).
    pub fn default_auto_reconnect() -> bool {
        true
    }

    /// Decode a base64 string and assert exactly 32 bytes.
    ///
    /// Shared by `server_public_key` and `psk` validation.
    pub fn decode_key_32(encoded: &str) -> Result<[u8; 32], String> {
        let bytes = STANDARD
            .decode(encoded)
            .map_err(|e| format!("invalid base64: {e}"))?;
        let arr: [u8; 32] = bytes
            .try_into()
            .map_err(|v: Vec<u8>| format!("expected 32 bytes, got {}", v.len()))?;
        Ok(arr)
    }
}

impl OptionalSection for VpnClientConfig {
    fn section_name() -> &'static str {
        const TOML_KEY: &str = "vpn";
        TOML_KEY
    }

    fn on_error() -> OnError {
        // VPN declared but invalid = startup abort.
        // Running without a tunnel when VPN was explicitly configured is a
        // security violation — never silently degrade to no tunnel.
        OnError::Fail
    }

    fn validate_enabled(&self) -> Result<(), ConfigError> {
        Self::decode_key_32(&self.server_public_key).map_err(|e| {
            ConfigError::validation(
                "vpn",
                format!("server_public_key: {e} (expected base64-encoded 32-byte X25519 key)"),
            )
        })?;

        if let Some(ref psk) = self.psk {
            Self::decode_key_32(psk).map_err(|e| {
                ConfigError::validation(
                    "vpn",
                    format!("psk: {e} (expected base64-encoded 32-byte key)"),
                )
            })?;
        }

        if self.keepalive_interval == 0 {
            return Err(ConfigError::validation("vpn", "keepalive_interval must be > 0"));
        }
        if self.handshake_timeout == 0 {
            return Err(ConfigError::validation("vpn", "handshake_timeout must be > 0"));
        }
        Ok(())
    }

    fn metadata() -> FeatureMetadata {
        FeatureMetadata {
            description: "WireGuard VPN client — routes all egress through the swelvpn tunnel",
            owner: "platform-team",
            deprecated_since: None,
        }
    }
}
