//! # edge-domain-vpn
//!
//! VPN port contracts for the domain layer.
//!
//! Provides the `TunnelManager` trait for connecting and disconnecting a VPN
//! tunnel, `VpnClientConfig` as an `OptionalSection` TOML toggle, a no-op
//! `NoopTunnelManager` for tests and non-unix platforms, and a
//! `VpnManagerFactory` whose SAF methods return `TunnelManager` implementations.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use edge_domain_vpn::{VpnClientConfig, VpnManagerFactory};
//! use swe_edge_configbuilder::{ConfigLoaderFactory, FeatureState, OptionalSection};
//!
//! # async fn example() {
//! let loader = ConfigLoaderFactory::create_loader_for_dir("config/");
//! match VpnClientConfig::load_optional(&loader).unwrap() {
//!     FeatureState::Enabled(_cfg) => {
//!         let mgr = VpnManagerFactory::noop_tunnel_manager();
//!         mgr.connect().await.unwrap();
//!     }
//!     FeatureState::Disabled => println!("VPN off"),
//! }
//! # }
//! ```

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;
mod spi;

pub use saf::TunnelManager;
pub use saf::TunnelStatus;
pub use saf::VpnClientConfig;
pub use saf::VpnError;
pub use saf::VpnManagerFactory;
pub use saf::VpnResult;
pub use saf::TUNNEL_MANAGER_SVC;
pub use saf::CLIENT_CONFIG_SVC;

pub use api::NoopTunnelManager;
