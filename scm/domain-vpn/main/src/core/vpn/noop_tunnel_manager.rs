//! `TunnelManager` implementation for `NoopTunnelManager`.

use async_trait::async_trait;

use crate::api::NoopTunnelManager;
use crate::api::TunnelManager;
use crate::api::TunnelStatus;
use crate::api::VpnClientConfig;
use crate::api::VpnError;
use crate::api::VpnManagerFactory;
use crate::api::VpnResult;

#[async_trait]
impl TunnelManager for NoopTunnelManager {
    async fn connect(&self) -> VpnResult<()> {
        Ok(())
    }

    async fn disconnect(&self) -> VpnResult<()> {
        Ok(())
    }

    async fn status(&self) -> VpnResult<TunnelStatus> {
        Ok(TunnelStatus::Connected)
    }

    async fn configure(&self, _config: &VpnClientConfig) -> Result<(), VpnError> {
        Ok(())
    }

    fn factory() -> VpnManagerFactory
    where
        Self: Sized,
    {
        VpnManagerFactory
    }
}
