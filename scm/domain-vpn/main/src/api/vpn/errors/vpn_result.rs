//! `VpnResult` — convenience alias for `Result<T, VpnError>`.

use crate::api::VpnError;

/// `Result` alias for VPN operations.
pub type VpnResult<T> = Result<T, VpnError>;
