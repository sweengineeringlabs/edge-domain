//! `OutboundRegistry` — keyed registry of outbound handles.

use crate::api::domain::errors::DomainError;
use crate::api::domain::types::{
    OutboundDeregisterRequest, OutboundDeregisterResponse, OutboundGetRequest, OutboundGetResponse,
    OutboundIsEmptyRequest, OutboundIsEmptyResponse, OutboundLenRequest, OutboundLenResponse,
    OutboundNamesRequest, OutboundNamesResponse, OutboundRegisterRequest, OutboundRegisterResponse,
};

/// Thread-safe registry of outbound handles keyed by name.
///
/// The canonical implementation is
/// [`MemoryOutboundRegistry`](crate::api::domain::types::MemoryOutboundRegistry).
pub trait OutboundRegistry: Send + Sync {
    /// The handle type stored in this registry.
    type Handle: Clone + Send + Sync;

    /// Register `handle` under `name`, replacing any existing entry.
    fn register(
        &self,
        req: OutboundRegisterRequest<Self::Handle>,
    ) -> Result<OutboundRegisterResponse, DomainError>;

    /// Remove the handle registered under `name`.
    fn deregister(
        &self,
        req: OutboundDeregisterRequest,
    ) -> Result<OutboundDeregisterResponse, DomainError>;

    /// Look up a handle by name. Returns a clone on hit, `None` on miss.
    fn get(
        &self,
        req: OutboundGetRequest,
    ) -> Result<OutboundGetResponse<Self::Handle>, DomainError>;

    /// Snapshot of registered names. Order is unspecified.
    fn names(&self, req: OutboundNamesRequest) -> Result<OutboundNamesResponse, DomainError>;

    /// Number of currently registered handles.
    fn len(&self, req: OutboundLenRequest) -> Result<OutboundLenResponse, DomainError>;

    /// Whether the registry holds no handles.
    fn is_empty(&self, req: OutboundIsEmptyRequest)
        -> Result<OutboundIsEmptyResponse, DomainError>;
}
